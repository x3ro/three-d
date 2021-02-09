
uniform vec4 surfaceColor;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

uniform vec3 ambientColor;

layout (std140) uniform DirectionalLightUniform
{
    DirectionalLight light;
};

in vec3 nor;
in vec3 pos;
in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(1.0);
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
	Surface surface = Surface(pos, normal, surfaceColor.rgb, diffuse_intensity, specular_intensity, specular_power);
    outColor = vec4(ambientColor * surfaceColor.rgb + calculate_directional_light(light, surface), surfaceColor.a);
}