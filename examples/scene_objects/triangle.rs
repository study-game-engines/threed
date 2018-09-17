
use gl;
use dust::*;
use std::rc::Rc;

pub struct Triangle {
    program: program::Program,
    model: surface::TriangleSurface
}

impl traits::Reflecting for Triangle
{
    fn reflect(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.model.render()?;
        Ok(())
    }
}

impl Triangle
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let positions: Vec<f32> = vec![
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0,// bottom left
            0.0,  0.5, 0.0 // top
        ];
        let colors: Vec<f32> = vec![
            1.0, 0.0, 0.0,   // bottom right
            0.0, 1.0, 0.0,   // bottom left
            0.0, 0.0, 1.0    // top
        ];
        let mesh = mesh::StaticMesh::create((0..3).collect(), att!["position" => (positions, 3), "color" => (colors, 3)]).unwrap();
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/color")?;
        let mut model = surface::TriangleSurface::create(gl, &mesh)?;
        model.add_attributes(&mesh, &program,&vec!["position", "color"])?;

        Ok(Rc::new(Triangle { program, model }))
    }
}
