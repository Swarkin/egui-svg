# egui-svg

a collection of crates that allow you to export a static, noninteractive svg from the output of egui.

### notes

- when adding new features, some are trivial to implement using native svg features, some require converting elements into paths and a small portion arent possible at all.
- your svg renderer, for example your browser, must support custom fonts.

### known issues

- text support is very primitive
- rounding cannot be different per corner
- textures arent supported
    - images arent supported
    - textured meshes arent supported
- clipping is not supported
- some elements are half a pixel off
- the default fonts i included are pretty big (~400kb) and its currently not possible to switch them out dynamically

### usage (html file)

- use the included html file
- paste the resulting egui.svg into the empty body
- open the html file in your browser
