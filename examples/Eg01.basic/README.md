
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
| 03             | staging  | Show the usage of staging buffer.                            |
| 04             | uniform  | Show the usage of uniform buffer.                            |
| 05             | texture  | Use sampler to render an image on screen.                    |
| 06             | box      | Render an 3D box object and use arrow keys to control its angle. |

### Example usage:

```sh
$ cargo run --bin 01
```

