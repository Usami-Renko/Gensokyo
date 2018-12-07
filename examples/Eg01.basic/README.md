
# Examples-basic

These examples show the basic usage of hakurei.

`snapshots` directory contains snapshot of each example.

## Running the examples:

```sh
$ cargo run --bin <example_number>
```

All available examples and corresponding `example_number` are listed in the following table:

| example_number | example  | description                                                  |
| -------------- | -------- | ------------------------------------------------------------ |
| 01             | triangle | Draw the basic triangle.                                     |
| 02             | index    | Draw the triangle with index buffer.                         |
| 03             | uniform  | Show the usage of uniform buffer.                            |
| 04             | texture  | Use sampler to render an image on screen.                    |
| 05             | cube     | Render an 3D cube object. Use arrow keys or mouse to control its rotation. |
| 06             | depth    | Use depth attachment in graphics pipeline. Use arrow keys to move the camera location.(Currently this example failed to adapt the window resize event since the depth image need to recreate.) |
| 07             | model( unavailable) | Load a simple model. Use arrow keys and mouse to control the camera. |

### Example usage:

```sh
$ cargo run --bin 01
```
