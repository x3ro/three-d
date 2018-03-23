use gl;
use std;

use utility;
use shader;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program
{
    pub fn from_source(vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Program, String>
    {
        use std::ffi::{CString};
        let vert_shader = shader::Shader::from_vert_source(
            &CString::new(vertex_shader_source).unwrap()
        ).unwrap();

        let frag_shader = shader::Shader::from_frag_source(
            &CString::new(fragment_shader_source).unwrap()
        ).unwrap();

        return Program::from_shaders( &[vert_shader, frag_shader] );
    }

    pub fn from_shaders(shaders: &[shader::Shader]) -> Result<Program, String>
    {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = utility::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
