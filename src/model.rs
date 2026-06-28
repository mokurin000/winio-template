//! Main GUI component for the QR code generator.

use winio::prelude::*;

use crate::Result;

/// Root component of the application UI.
pub struct MainModel {
    /// The main application window.
    window: Child<Window>,
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
                size: Size::new(800.0, 600.0),

                #[cfg(all(windows, feature = "winui"))]
                backdrop: Backdrop::Mica,
            },
        }

        window.show()?;

        Ok(Self { window })
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
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        // update the window
        update_children!(self.window)
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        // deal with custom messages
        match message {
            MainMessage::Noop => Ok(false),
            MainMessage::Resize => Ok(false),
            MainMessage::ThemeChanged => Ok(false),
            MainMessage::Close => {
                // the root component output stops the application
                sender.output(());
                // need not to call `render`
                Ok(false)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        let _csize = self.window.client_size()?;

        // add your UI layout here
        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        Ok(self.window.render()?)
    }
}
