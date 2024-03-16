### Run

```bash
cargo run --features "bevy/dynamic_linking"
```


## notes
get window size
  ```rust
  window: Query<&Window>
  let win = window.single();
  let half_height = -win.height() * 0.50f32;
  let half_width = win.width() * 0.50f32;
  ```

