use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys::Float32Array, HtmlCanvasElement, WebGlProgram, WebGlRenderingContext, WebGlShader};


// Вершинный шейдер
const VERTEX_SHADER: &str = r#"
attribute vec2 position; // Изменим тип на vec2, если вы передаете только x и y
void main() {
    gl_Position = vec4(position, 0.0, 1.0); // Теперь это корректно, так как position имеет два компонента
}
"#;

// Фрагментный шейдер
const FRAGMENT_SHADER: &str = r#"
precision mediump float;
void main() {
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); // Красный цвет
}
"#;

const VERTICES: &[f32] = &[
    -0.5, -0.5, // Нижний левый угол
     0.5, -0.5, // Нижний правый угол
    -0.5,  0.5, // Верхний левый угол
     0.5,  0.5, // Верхний правый угол
];



#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console::log_1(&"start function called".into());
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document
        .get_element_by_id("webgl-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        VERTEX_SHADER,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER,
    )?;

    let program = create_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    // Установите цвет фона (синий в этом случае)
    context.clear_color(0.0, 0.0, 1.0, 1.0); // Синий фон
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    // Здесь будет ваш код для создания и отрисовки сетки
    //
    let vertices_array = Float32Array::from(VERTICES);
    let vertex_buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );
    let pos_attr = context.get_attrib_location(&program, "position") as u32;
    context.vertex_attrib_pointer_with_i32(pos_attr, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(pos_attr);

    // Очищаем экран, используя синий цвет
    context.clear_color(0.0, 0.0, 1.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // Рисуем красную сетку (в этом случае просто линии)
    context.draw_arrays(WebGlRenderingContext::LINE_LOOP, 0, 4);

    Ok(())
}

// Функция для компиляции шейдера
fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

// Функция для создания и линковки WebGL программы
fn create_program(
    context: &WebGlRenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader program"))?;

    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program")))
    }
}
