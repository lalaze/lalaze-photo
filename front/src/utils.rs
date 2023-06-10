use web_sys::window;

pub fn set_zoom() {
  let window = window().expect("window is undefined");
  let width = window.outer_width().unwrap().as_f64();
  let s = match width {
    Some(value) => value / 1920 as f64,
    None => 1.0,
  };
  let document =  window.document()
      .expect("document is undefined");
  document.body().unwrap().set_attribute("style", &format!("zoom: {}", s)).unwrap();
}