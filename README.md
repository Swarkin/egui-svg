# egui-svg

a collection of crates that allow you to export a static, noninteractive svg from the output of egui.

| egui  | egui-svg |
| --- | --- |
| ![egui](https://github.com/user-attachments/assets/340778c4-c5f5-4426-a1a5-0dda6ffd006b) | ![egui-svg](https://github.com/user-attachments/assets/17d81d58-316e-4766-8a1b-49d991270c2f) |

## notes

- when adding new features, most are trivial to implement using native svg features, some require converting elements into paths and a small portion arent possible at all.
- your svg renderer, for example your browser, must support custom fonts.

## known issues

- text support is very primitive
- rounding cannot be different per corner
- textures arent supported
    - images arent supported
    - textured meshes arent supported
- clipping is not supported
- some elements are half a pixel off
- the default fonts i included are pretty big (~400kb) and its currently not possible to switch them out dynamically

## usage (html file)

- use the included html file
- paste the resulting egui.svg into the empty body
- open the html file in your browser
