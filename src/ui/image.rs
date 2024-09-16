use eyre::Result;
use ratatui::{layout::Rect, Frame};
use ratatui_image::{picker::Picker, StatefulImage};

// TODO: Split the image part away from the ui part
pub fn image(f: &mut Frame, area: Rect) -> Result<()> {
    // Should use Picker::from_termios(), to get the font size,
    // but we can't put that here because that would break doctests!
    let mut picker = Picker::new((8, 12));
    // Guess the protocol.
    picker.guess_protocol();

    // Load an image with the image crate.
    let dyn_img = image::ImageReader::open("./assets/daiq.jpg")?.decode()?;

    // Create the Protocol which will be used by the widget.
    let mut image = picker.new_resize_protocol(dyn_img);

    let widget = StatefulImage::new(None);

    f.render_stateful_widget(widget, area, &mut image);
    Ok(())
}
