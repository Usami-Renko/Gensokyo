This is a rendering engine based on Vulkan and ash. 

The main purpose of writing this project is to learn Vulkan and implement graphic demos.

### Overview

[gensokyo-vulkan](./gensokyo-vulkan) contains the wrapper functions to simplify Vulkan API.

[gensokyo-macros](./gensokyo-macros) defines some useful macros.

[gensokyo](./gensokyo) contains the components of an engine(input handle, game loop, glTF, camera..etc).

[examples](./examples) contains all demos.

### Roadmap

- [x] Load configuration from toml
- [ ] Memory management based on linked list
- [ ] Text rendering
- [ ] Image
  - [x] render png, jpg format
  - [ ] render ktx format(WIP)
- [ ] glTF
  - [x] Vertex attributes
  - [x] Materials
  - [ ] Texture
  - [ ] Animation
  - [ ] Skin