pub fn copy_to_clipboard(content: &str) {
    use clipboard::{ClipboardContext, ClipboardProvider};

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content.to_owned()).unwrap();
}