//! Main GUI component for the QR code generator.

use winio::prelude::*;

use crate::Result;

/// Root component of the application UI.
pub struct MainModel {
    /// The main application window.
    window: Child<Window>,
    /// label
    label: Child<Label>,
    /// count up
    countup: Child<Button>,
    /// count down
    countdown: Child<Button>,

    count: i8,
}

pub enum MainMessage {
    /// Nothing to do
    Noop,
    /// Main window has been resized
    Resize,
    /// Theme changed
    ThemeChanged,
    /// Close main window
    Close,
    /// Count plus by one
    CountUp,
    /// Count minus by one
    CountDown,
}

impl Component for MainModel {
    type Error = color_eyre::Report;
    type Event = ();
    type Init<'a> = ();
    type Message = MainMessage;

    async fn init(_init: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        // Note: color-eyre does not enable VT100 on Windows on its own
        color_eyre::install()?;

        init! {
            window: Window = (()) => {
                text: "Main window",
                size: Size::new(300.0, 100.0),

                #[cfg(all(windows, feature = "winui"))]
                backdrop: Backdrop::Mica,
            },
            label: Label = (&window) => {
                text: "0",
            },
            countup: Button = (&window) => {
                text: "+",
            },
            countdown: Button = (&window) => {
                text: "-",
            },
        }

        window.show()?;

        Ok(Self {
            window,
            label,
            countup,
            countdown,

            count: 0,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        // listen to events
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Resize => MainMessage::Resize,
                WindowEvent::Close => MainMessage::Close,
                WindowEvent::ThemeChanged => MainMessage::ThemeChanged,
            },
            self.countup => {
                ButtonEvent::Click => MainMessage::CountUp,
            },
            self.countdown => {
                ButtonEvent::Click => MainMessage::CountDown,
            },
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        // update the window and functional children
        update_children!(self.window, self.countup,)
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        // deal with custom messages
        match message {
            MainMessage::Noop => Ok(false),
            MainMessage::ThemeChanged => Ok(false),
            MainMessage::Resize => Ok(true),
            MainMessage::Close => {
                // the root component output stops the application
                sender.output(());
                // need not to call `render`
                Ok(false)
            }
            MainMessage::CountUp => {
                self.count += 1;
                self.label.set_text(format!("{}", self.count))?;
                Ok(true)
            }
            MainMessage::CountDown => {
                self.count -= 1;
                self.label.set_text(format!("{}", self.count))?;
                Ok(true)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        let csize = self.window.client_size()?;

        let mut buttons = layout! {
            StackPanel::new(Orient::Horizontal),
            self.countup => {
                halign: HAlign::Center,
            },
            self.countdown => {
                halign: HAlign::Center,
            },
        };
        let mut layout = layout! {
            StackPanel::new(Orient::Vertical),
            self.label => {
                halign: HAlign::Center,
            },
            buttons => {
                halign: HAlign::Center,
            },
        };
        layout.set_size(csize)?;
        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        Ok(self.window.render()?)
    }
}
