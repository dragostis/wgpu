#include <stdio.h>
#include "./../../wgpu-bindings/wgpu.h"

#define WGPU_TARGET_MACOS 1
#define WGPU_TARGET_LINUX 2
#define WGPU_TARGET_WINDOWS 3

#if WGPU_TARGET == WGPU_TARGET_MACOS
#include <QuartzCore/CAMetalLayer.h>
#include <Foundation/Foundation.h>
#endif

#include <GLFW/glfw3.h>
#if WGPU_TARGET == WGPU_TARGET_MACOS
#define GLFW_EXPOSE_NATIVE_COCOA
#elif WGPU_TARGET == WGPU_TARGET_LINUX
#define GLFW_EXPOSE_NATIVE_X11
#define GLFW_EXPOSE_NATIVE_WAYLAND
#elif WGPU_TARGET == WGPU_TARGET_WINDOWS
#define GLFW_EXPOSE_NATIVE_WIN32
#endif
#include <GLFW/glfw3native.h>

#define BLEND_STATES_LENGTH (1)
#define ATTACHMENTS_LENGTH (1)
#define RENDER_PASS_ATTACHMENTS_LENGTH (1)
#define BIND_GROUP_LAYOUTS_LENGTH (1)

WGPUByteArray read_file(const char *name)
{
    FILE *file = fopen(name, "rb");
    fseek(file, 0, SEEK_END);
    long length = ftell(file);
    unsigned char *bytes = malloc(length);
    fseek(file, 0, SEEK_SET);
    fread(bytes, 1, length, file);
    fclose(file);
    WGPUByteArray ret = {
        .bytes = bytes,
        .length = length,
    };
    return ret;
}

int main()
{
    WGPUInstanceId instance = wgpu_create_instance();
    WGPUAdapterDescriptor adapter_desc = {
        .power_preference = WGPUPowerPreference_LowPower,
    };
    WGPUAdapterId adapter = wgpu_instance_get_adapter(instance, &adapter_desc);
    WGPUDeviceDescriptor device_desc = {
        .extensions = {
            .anisotropic_filtering = false,
        },
    };
    WGPUDeviceId device = wgpu_adapter_create_device(adapter, &device_desc);

    WGPUShaderModuleDescriptor vertex_shader_desc = {
        .code = read_file("./../../data/hello_triangle.vert.spv"),
    };
    WGPUShaderModuleId vertex_shader = wgpu_device_create_shader_module(device, &vertex_shader_desc);
    WGPUPipelineStageDescriptor vertex_stage = {
        .module = vertex_shader,
        .entry_point = "main",
    };

    WGPUShaderModuleDescriptor fragment_shader_desc = {
        .code = read_file("./../../data/hello_triangle.frag.spv"),
    };
    WGPUShaderModuleId fragment_shader = wgpu_device_create_shader_module(device, &fragment_shader_desc);
    WGPUPipelineStageDescriptor fragment_stage = {
        .module = fragment_shader,
        .entry_point = "main",
    };

    WGPUBindGroupLayoutDescriptor bind_group_layout_desc = {
        .bindings = NULL,
        .bindings_length = 0,
    };
    WGPUBindGroupLayoutId bind_group_layout = wgpu_device_create_bind_group_layout(device, &bind_group_layout_desc);

    WGPUBindGroupLayoutId bind_group_layouts[BIND_GROUP_LAYOUTS_LENGTH] = { bind_group_layout };

    WGPUPipelineLayoutDescriptor pipeline_layout_desc = {
        .bind_group_layouts = bind_group_layouts,
        .bind_group_layouts_length = BIND_GROUP_LAYOUTS_LENGTH,
    };
    WGPUPipelineLayoutId pipeline_layout = wgpu_device_create_pipeline_layout(device, &pipeline_layout_desc);

    WGPUBlendDescriptor blend_alpha = {
        .src_factor = WGPUBlendFactor_One,
        .dst_factor = WGPUBlendFactor_Zero,
        .operation = WGPUBlendOperation_Add,
    };
    WGPUBlendDescriptor blend_color = {
        .src_factor = WGPUBlendFactor_One,
        .dst_factor = WGPUBlendFactor_Zero,
        .operation = WGPUBlendOperation_Add,
    };
    WGPUColorStateDescriptor color_state_desc = {
        .format = WGPUTextureFormat_Bgra8Unorm,
        .alpha = blend_alpha,
        .color = blend_color,
        .write_mask = WGPUColorWriteFlags_ALL,
    };
    WGPURasterizationStateDescriptor rasterization_state = {
        .front_face = WGPUFrontFace_Ccw,
        .cull_mode = WGPUCullMode_None,
        .depth_bias = 0,
        .depth_bias_slope_scale = 0.0,
        .depth_bias_clamp = 0.0,
    };
    WGPUVertexBufferStateDescriptor vertex_buffer_state = {
        .index_format = WGPUIndexFormat_Uint16,
        .vertex_buffers = NULL,
        .vertex_buffers_count = 0,
    };
    WGPURenderPipelineDescriptor render_pipeline_desc = {
        .layout = pipeline_layout,
        .vertex_stage = vertex_stage,
        .fragment_stage = fragment_stage,
        .rasterization_state = rasterization_state,
        .primitive_topology = WGPUPrimitiveTopology_TriangleList,
        .color_states = &color_state_desc,
        .color_states_length = 1,
        .depth_stencil_state = NULL,
        .vertex_buffer_state = vertex_buffer_state,
        .sample_count = 1,
    };

    WGPURenderPipelineId render_pipeline = wgpu_device_create_render_pipeline(device, &render_pipeline_desc);

    if (!glfwInit())
    {
        printf("Cannot initialize glfw");
        return 1;
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    GLFWwindow *window = glfwCreateWindow(640, 480, "wgpu with glfw", NULL, NULL);

    if (!window)
    {
        printf("Cannot create window");
        return 1;
    }

    WGPUSurfaceId surface = {};

#if WGPU_TARGET == WGPU_TARGET_MACOS
    {
        id metal_layer = NULL;
        NSWindow *ns_window = glfwGetCocoaWindow(window);
        CALayer *layer = ns_window.contentView.layer;
        [ns_window.contentView setWantsLayer:YES];
        metal_layer = [CAMetalLayer layer];
        [ns_window.contentView setLayer:metal_layer];
        surface = wgpu_instance_create_surface_from_macos_layer(instance, metal_layer);
    }
#elif WGPU_TARGET == WGPU_TARGET_LINUX
    {
        Display* x11_display = glfwGetX11Display();
        Window x11_window = glfwGetX11Window(window);
        surface = wgpu_instance_create_surface_from_xlib(instance, (const void**)x11_display, x11_window);
    }
#elif WGPU_TARGET == WGPU_TARGET_WINDOWS
    {
		HWND hwnd = glfwGetWin32Window(window);
		HINSTANCE hinstance = GetModuleHandle(NULL);
		surface = wgpu_instance_create_surface_from_windows_hwnd(instance, hinstance, hwnd);
    }
#endif

    WGPUSwapChainDescriptor swap_chain_desc = {
        .usage = WGPUTextureUsageFlags_OUTPUT_ATTACHMENT,
        .format = WGPUTextureFormat_Bgra8Unorm,
        .width = 640,
        .height = 480,
    };
    WGPUSwapChainId swap_chain = wgpu_device_create_swap_chain(device, surface, &swap_chain_desc);

    while (!glfwWindowShouldClose(window))
    {
        WGPUSwapChainOutput next_texture = wgpu_swap_chain_get_next_texture(swap_chain);
        WGPUCommandEncoderDescriptor cmd_encoder_desc = { .todo = 0 };
        WGPUCommandEncoderId cmd_encoder = wgpu_device_create_command_encoder(device, &cmd_encoder_desc);
        WGPURenderPassColorAttachmentDescriptor_TextureViewId color_attachments[ATTACHMENTS_LENGTH] = {
            {
                .attachment = next_texture.view_id,
                .load_op = WGPULoadOp_Clear,
                .store_op = WGPUStoreOp_Store,
                .clear_color = WGPUColor_GREEN,
            },
        };
        WGPURenderPassDescriptor rpass_desc = {
            .color_attachments = color_attachments,
            .color_attachments_length = RENDER_PASS_ATTACHMENTS_LENGTH,
            .depth_stencil_attachment = NULL,
        };
        WGPURenderPassId rpass = wgpu_command_encoder_begin_render_pass(cmd_encoder, rpass_desc);
        wgpu_render_pass_set_pipeline(rpass, render_pipeline);
        wgpu_render_pass_draw(rpass, 3, 1, 0, 0);
        WGPUCommandBufferId cmd_buf = wgpu_render_pass_end_pass(rpass);
        WGPUQueueId queue = wgpu_device_get_queue(device);
        wgpu_queue_submit(queue, &cmd_buf, 1);
        wgpu_swap_chain_present(swap_chain);

        glfwPollEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();

    return 0;
}
