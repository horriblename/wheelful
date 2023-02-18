use conv::prelude::*;
use gdk::EventMask;
use gio::prelude::*;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

#[derive(Debug)]
struct Wheel {
    center: Option<(f64, f64)>,
    actions: Vec<ActionBubble>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActionBubble {
    name: String,
    icon: String,
    command: Option<String>,
    subwheel: Option<Vec<ActionBubble>>,
}

impl Wheel {
    fn new() -> Self {
        let actions: Vec<ActionBubble> = serde_yaml::from_str(include_str!("default.yaml"))
            .expect("Error parsing yaml configuration!");
        Wheel {
            center: None,
            actions,
        }
    }

    fn draw(&self, widget: &gtk::DrawingArea, context: &gdk::cairo::Context) -> Inhibit {
        // if self.center.is_none() {
        //     return Inhibit(false);
        // }
        let radius: f64 = 30.0;
        let distance: f64 = 80.0;
        let rbubble: f64 = 20.0;
        let center = self.center.unwrap_or_else(|| {
            let width: f64 = widget.allocated_width().into();
            let height: f64 = widget.allocated_height().into();
            (width / 2.0, height / 2.0)
        });
        let rotation: f64 = 2.0 * PI / self.actions.len().value_as::<f64>().unwrap();

        let style_context = widget.style_context();
        let color = style_context.color(style_context.state());
        // context.set_source_rgba(color.red(), color.green(), color.blue(), color.alpha());
        context.set_source_rgba(1.0, 0.0, 1.0, 0.5);

        context.arc(center.0.into(), center.1.into(), radius, 0.0, 2.0 * PI);
        context.fill().unwrap();

        for i in 0..self.actions.len() {
            let (sin, cos) = f64::sin_cos(rotation * i.value_as::<f64>().unwrap());
            let xc = center.0 + (distance * sin);
            let yc = center.1 - (distance * cos);
            context.arc(xc, yc, rbubble, 0.0, 2.0 * PI);
            context.fill().unwrap();
        }

        println!("drawn");
        Inhibit(false)
    }

    fn on_button(&mut self, widget: &gtk::DrawingArea, event: &gdk::EventButton) -> Inhibit {
        self.center = Some(event.position());

        println!("received click");
        Inhibit(false)
    }
}

fn canvas_draw_callback(widget: &gtk::DrawingArea, context: &gdk::cairo::Context) -> Inhibit {
    let width: f64 = widget.allocated_width().into();
    let height: f64 = widget.allocated_height().into();
    let style_context = widget.style_context();
    let color = style_context.color(style_context.state());

    context.set_source_rgba(color.red(), color.green(), color.blue(), color.alpha());

    gtk::render_background(&style_context, context, 0.0, 0.0, width, height);

    context.arc(
        width / 2.0,
        height / 2.0,
        if width < height { width } else { height } / 2.0,
        0.0,
        2.0 * PI,
    );
    context.fill().unwrap();

    println!("drawing area is {}x{}", width, height);

    Inhibit(false)
}

fn canvas_on_button(widget: &gtk::DrawingArea, event: &gdk::EventButton) -> Inhibit {
    Inhibit(false)
}

fn canvas_on_button_release(widget: &gtk::DrawingArea, event: &gdk::EventButton) -> Inhibit {
    Inhibit(false)
}

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);

    // Order below normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    // gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, 40);
    // gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, 40);
    // gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, 20);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk_layer_shell::Edge::Left, true),
        (gtk_layer_shell::Edge::Right, true),
        (gtk_layer_shell::Edge::Top, true),
        (gtk_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk_layer_shell::set_anchor(&window, anchor, state);
    }

    // how am I supposed to do this???
    let mut ev_mask = EventMask::empty();
    ev_mask.set(EventMask::TOUCH_MASK, true);
    ev_mask.set(EventMask::BUTTON_PRESS_MASK, true);
    ev_mask.set(EventMask::BUTTON_RELEASE_MASK, true);
    ev_mask.set(EventMask::BUTTON_MOTION_MASK, true);

    let canvas = gtk::DrawingArea::builder()
        .opacity(0.5)
        .hexpand(true)
        .vexpand(true)
        .events(ev_mask)
        .build();

    let wheel = Rc::new(RefCell::new(Wheel::new()));

    let wheel_rc = wheel.clone();
    canvas.connect_draw(move |widget, context| {
        let wheel = wheel_rc.borrow();
        wheel.draw(widget, context);
        Inhibit(false)
    });

    let wheel_rc = wheel.clone();
    canvas.connect_button_press_event(move |widget, context| {
        let mut wheel = wheel_rc.borrow_mut();
        wheel.on_button(widget, context);
        Inhibit(false)
    });
    canvas.connect_button_release_event(canvas_on_button_release);

    window.add(&canvas);
    window.set_border_width(12);
    window.show_all()
}

fn main() {
    let application = gtk::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}

#[cfg(test)]
mod test {
    use crate::ActionBubble;

    #[test]
    fn test_yaml_read() {
        let values: Vec<ActionBubble> = serde_yaml::from_str(include_str!("default.yaml")).unwrap();
        assert_eq!(values[0].name, "Music");
        println!("{values:?}");
    }
}
