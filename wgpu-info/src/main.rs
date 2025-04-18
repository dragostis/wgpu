#[cfg(not(target_arch = "wasm32"))]
mod inner {
    use std::{
        mem::size_of,
        process::{exit, Command},
        time::Instant,
    };

    // Lets keep these on one line
    #[rustfmt::skip]
    const TEXTURE_FORMAT_LIST: [wgpu::TextureFormat; 113] = [
        wgpu::TextureFormat::R8Unorm,
        wgpu::TextureFormat::R8Snorm,
        wgpu::TextureFormat::R8Uint,
        wgpu::TextureFormat::R8Sint,
        wgpu::TextureFormat::R16Uint,
        wgpu::TextureFormat::R16Sint,
        wgpu::TextureFormat::R16Unorm,
        wgpu::TextureFormat::R16Snorm,
        wgpu::TextureFormat::R16Float,
        wgpu::TextureFormat::Rg8Unorm,
        wgpu::TextureFormat::Rg8Snorm,
        wgpu::TextureFormat::Rg8Uint,
        wgpu::TextureFormat::Rg8Sint,
        wgpu::TextureFormat::R32Uint,
        wgpu::TextureFormat::R32Sint,
        wgpu::TextureFormat::R32Float,
        wgpu::TextureFormat::Rg16Uint,
        wgpu::TextureFormat::Rg16Sint,
        wgpu::TextureFormat::Rg16Unorm,
        wgpu::TextureFormat::Rg16Snorm,
        wgpu::TextureFormat::Rg16Float,
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureFormat::Rgba8Snorm,
        wgpu::TextureFormat::Rgba8Uint,
        wgpu::TextureFormat::Rgba8Sint,
        wgpu::TextureFormat::Bgra8Unorm,
        wgpu::TextureFormat::Bgra8UnormSrgb,
        wgpu::TextureFormat::Rgb10a2Unorm,
        wgpu::TextureFormat::Rg11b10Float,
        wgpu::TextureFormat::Rg32Uint,
        wgpu::TextureFormat::Rg32Sint,
        wgpu::TextureFormat::Rg32Float,
        wgpu::TextureFormat::Rgba16Uint,
        wgpu::TextureFormat::Rgba16Sint,
        wgpu::TextureFormat::Rgba16Unorm,
        wgpu::TextureFormat::Rgba16Snorm,
        wgpu::TextureFormat::Rgba16Float,
        wgpu::TextureFormat::Rgba32Uint,
        wgpu::TextureFormat::Rgba32Sint,
        wgpu::TextureFormat::Rgba32Float,
        //wgpu::TextureFormat::Stencil8,
        wgpu::TextureFormat::Depth16Unorm,
        wgpu::TextureFormat::Depth32Float,
        wgpu::TextureFormat::Depth32FloatStencil8,
        wgpu::TextureFormat::Depth24Plus,
        wgpu::TextureFormat::Depth24PlusStencil8,
        wgpu::TextureFormat::Rgb9e5Ufloat,
        wgpu::TextureFormat::Bc1RgbaUnorm,
        wgpu::TextureFormat::Bc1RgbaUnormSrgb,
        wgpu::TextureFormat::Bc2RgbaUnorm,
        wgpu::TextureFormat::Bc2RgbaUnormSrgb,
        wgpu::TextureFormat::Bc3RgbaUnorm,
        wgpu::TextureFormat::Bc3RgbaUnormSrgb,
        wgpu::TextureFormat::Bc4RUnorm,
        wgpu::TextureFormat::Bc4RSnorm,
        wgpu::TextureFormat::Bc5RgUnorm,
        wgpu::TextureFormat::Bc5RgSnorm,
        wgpu::TextureFormat::Bc6hRgbUfloat,
        wgpu::TextureFormat::Bc6hRgbSfloat,
        wgpu::TextureFormat::Bc7RgbaUnorm,
        wgpu::TextureFormat::Bc7RgbaUnormSrgb,
        wgpu::TextureFormat::Etc2Rgb8Unorm,
        wgpu::TextureFormat::Etc2Rgb8UnormSrgb,
        wgpu::TextureFormat::Etc2Rgb8A1Unorm,
        wgpu::TextureFormat::Etc2Rgb8A1UnormSrgb,
        wgpu::TextureFormat::Etc2Rgba8Unorm,
        wgpu::TextureFormat::Etc2Rgba8UnormSrgb,
        wgpu::TextureFormat::EacR11Unorm,
        wgpu::TextureFormat::EacR11Snorm,
        wgpu::TextureFormat::EacRg11Unorm,
        wgpu::TextureFormat::EacRg11Snorm,
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B4x4, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B4x4, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B4x4, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B5x4, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B5x4, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B5x4, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B5x5, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B5x5, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B5x5, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B6x5, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B6x5, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B6x5, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B6x6, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B6x6, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B6x6, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x5, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x5, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x5, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x6, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x6, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x6, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x8, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x8, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B8x8, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x5, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x5, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x5, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x6, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x6, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x6, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x8, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x8, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x8, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x10, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x10, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B10x10, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B12x10, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B12x10, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B12x10, channel: wgpu::AstcChannel::Hdr },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B12x12, channel: wgpu::AstcChannel::Unorm },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B12x12, channel: wgpu::AstcChannel::UnormSrgb },
        wgpu::TextureFormat::Astc { block: wgpu::AstcBlock::B12x12, channel: wgpu::AstcChannel::Hdr },
    ];

    // Lets keep these on one line
    #[rustfmt::skip]
    fn print_info_from_adapter(adapter: &wgpu::Adapter, idx: usize) {
        let info = adapter.get_info();
        let downlevel = adapter.get_downlevel_capabilities();
        let features = adapter.features();
        let limits = adapter.limits();
    
        println!("Adapter {}:", idx);
        println!("\t   Backend: {:?}", info.backend);
        println!("\t      Name: {:?}", info.name);
        println!("\t  VendorID: {:?}", info.vendor);
        println!("\t  DeviceID: {:?}", info.device);
        println!("\t      Type: {:?}", info.device_type);
        println!("\t    Driver: {:?}", info.driver);
        println!("\tDriverInfo: {:?}", info.driver);
        println!("\t Compliant: {:?}", downlevel.is_webgpu_compliant());
        println!("\tFeatures:");
        for i in 0..(size_of::<wgpu::Features>() * 8) {
            let bit = wgpu::Features::from_bits(1 << i as u64);
            if let Some(bit) = bit {
                if wgpu::Features::all().contains(bit) {
                    println!("\t\t{:>63} {}", format!("{:?}:", bit), features.contains(bit));
                }
            }
        }
    
        println!("\tLimits:");
        let wgpu::Limits {
            max_texture_dimension_1d,
            max_texture_dimension_2d,
            max_texture_dimension_3d,
            max_texture_array_layers,
            max_bind_groups,
            max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage,
            max_samplers_per_shader_stage,
            max_storage_buffers_per_shader_stage,
            max_storage_textures_per_shader_stage,
            max_uniform_buffers_per_shader_stage,
            max_uniform_buffer_binding_size,
            max_storage_buffer_binding_size,
            max_buffer_size,
            max_vertex_buffers,
            max_vertex_attributes,
            max_vertex_buffer_array_stride,
            max_push_constant_size,
            min_uniform_buffer_offset_alignment,
            min_storage_buffer_offset_alignment,
            max_inter_stage_shader_components,
            max_compute_workgroup_storage_size,
            max_compute_invocations_per_workgroup,
            max_compute_workgroup_size_x,
            max_compute_workgroup_size_y,
            max_compute_workgroup_size_z,
            max_compute_workgroups_per_dimension,
        } = limits;
        println!("\t\t                        Max Texture Dimension 1d: {}", max_texture_dimension_1d);
        println!("\t\t                        Max Texture Dimension 2d: {}", max_texture_dimension_2d);
        println!("\t\t                        Max Texture Dimension 3d: {}", max_texture_dimension_3d);
        println!("\t\t                        Max Texture Array Layers: {}", max_texture_array_layers);
        println!("\t\t                                 Max Bind Groups: {}", max_bind_groups);
        println!("\t\t Max Dynamic Uniform Buffers Per Pipeline Layout: {}", max_dynamic_uniform_buffers_per_pipeline_layout);
        println!("\t\t Max Dynamic Storage Buffers Per Pipeline Layout: {}", max_dynamic_storage_buffers_per_pipeline_layout);
        println!("\t\t           Max Sampled Textures Per Shader Stage: {}", max_sampled_textures_per_shader_stage);
        println!("\t\t                   Max Samplers Per Shader Stage: {}", max_samplers_per_shader_stage);
        println!("\t\t            Max Storage Buffers Per Shader Stage: {}", max_storage_buffers_per_shader_stage);
        println!("\t\t           Max Storage Textures Per Shader Stage: {}", max_storage_textures_per_shader_stage);
        println!("\t\t            Max Uniform Buffers Per Shader Stage: {}", max_uniform_buffers_per_shader_stage);
        println!("\t\t                 Max Uniform Buffer Binding Size: {}", max_uniform_buffer_binding_size);
        println!("\t\t                 Max Storage Buffer Binding Size: {}", max_storage_buffer_binding_size);
        println!("\t\t                                 Max Buffer Size: {}", max_buffer_size);
        println!("\t\t                              Max Vertex Buffers: {}", max_vertex_buffers);
        println!("\t\t                           Max Vertex Attributes: {}", max_vertex_attributes);
        println!("\t\t                  Max Vertex Buffer Array Stride: {}", max_vertex_buffer_array_stride);
        println!("\t\t                          Max Push Constant Size: {}", max_push_constant_size);
        println!("\t\t             Min Uniform Buffer Offset Alignment: {}", min_uniform_buffer_offset_alignment);
        println!("\t\t             Min Storage Buffer Offset Alignment: {}", min_storage_buffer_offset_alignment);
        println!("\t\t                Max Inter-Stage Shader Component: {}", max_inter_stage_shader_components);
        println!("\t\t              Max Compute Workgroup Storage Size: {}", max_compute_workgroup_storage_size);
        println!("\t\t           Max Compute Invocations Per Workgroup: {}", max_compute_invocations_per_workgroup);
        println!("\t\t                    Max Compute Workgroup Size X: {}", max_compute_workgroup_size_x);
        println!("\t\t                    Max Compute Workgroup Size Y: {}", max_compute_workgroup_size_y);
        println!("\t\t                    Max Compute Workgroup Size Z: {}", max_compute_workgroup_size_z);
        println!("\t\t            Max Compute Workgroups Per Dimension: {}", max_compute_workgroups_per_dimension);
    
        println!("\tDownlevel Properties:");
        let wgpu::DownlevelCapabilities {
            shader_model,
            limits: _,
            flags,
        } = downlevel;
        println!("\t\t                       Shader Model: {:?}", shader_model);
        for i in 0..(size_of::<wgpu::DownlevelFlags>() * 8) {
            let bit = wgpu::DownlevelFlags::from_bits(1 << i as u64);
            if let Some(bit) = bit {
                if wgpu::DownlevelFlags::all().contains(bit) {
                    println!("\t\t{:>36} {}", format!("{:?}:", bit), flags.contains(bit));
                }
            }
        }

        println!("\tTexture Format Features:      ┌──────────┬──────────┬──────────Allowed┬Usages───────────┬───────────────────┐ ┌────────────┬─────────────┬──────────────Feature┬Flags───────────────┬─────────────────┐");
        for format in TEXTURE_FORMAT_LIST {
            let features = adapter.get_texture_format_features(format);
            let format_name = match format {
                wgpu::TextureFormat::Astc { block, channel } => {
                    format!("Astc{block:?}{channel:?}:")
                }
                _ => {
                    format!("{format:?}:")
                }
            };
            print!("\t\t{:>21}", format_name);
            for i in 0..(size_of::<wgpu::TextureUsages>() * 8) {
                let bit = wgpu::TextureUsages::from_bits(1 << i as u32);
                if let Some(bit) = bit {
                    print!(" │ ");
                    if features.allowed_usages.contains(bit) {
                        print!("{bit:?}");
                    }
                    else {
                        let length = format!("{bit:?}").len();
                        print!("{}", " ".repeat(length))
                    }
                }
            }
            print!(" │ │ ");
            for i in 0..(size_of::<wgpu::TextureFormatFeatureFlags>() * 8) {
                let bit = wgpu::TextureFormatFeatureFlags::from_bits(1 << i as u32);
                if let Some(bit) = bit {
                    if i != 0 {
                        print!(" │ ")
                    }
                    if features.flags.contains(bit) {
                        print!("{bit:?}");
                    }
                    else {
                        let length = format!("{bit:?}").len();
                        print!("{}", " ".repeat(length))
                    }
                }
            }
            println!(" │");
        }
        println!("\t                              └──────────┴──────────┴─────────────────┴─────────────────┴───────────────────┘ └────────────┴─────────────┴─────────────────────┴────────────────────┴─────────────────┘");
    }

    pub fn main() {
        env_logger::init();
        let args: Vec<_> = std::env::args().skip(1).collect();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapters: Vec<_> = instance.enumerate_adapters(wgpu::Backends::all()).collect();
        let adapter_count = adapters.len();

        if args.is_empty() {
            for (idx, adapter) in adapters.into_iter().enumerate() {
                print_info_from_adapter(&adapter, idx)
            }
        } else {
            let all_start = Instant::now();

            for (idx, adapter) in adapters.into_iter().enumerate() {
                let adapter_start_time = Instant::now();
                let idx = idx + 1;
                let info = adapter.get_info();
                println!(
                    "=========== TESTING {} on {:?} ({} of {}) ===========",
                    info.name, info.backend, idx, adapter_count
                );
                let exit_status = Command::new(&args[0])
                    .args(&args[1..])
                    .env("WGPU_ADAPTER_NAME", &info.name)
                    .env(
                        "WGPU_BACKEND",
                        match info.backend {
                            wgpu::Backend::Empty => unreachable!(),
                            wgpu::Backend::Vulkan => "vulkan",
                            wgpu::Backend::Metal => "metal",
                            wgpu::Backend::Dx12 => "dx12",
                            wgpu::Backend::Dx11 => "dx11",
                            wgpu::Backend::Gl => "gl",
                            wgpu::Backend::BrowserWebGpu => "webgpu",
                        },
                    )
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                let adapter_time = adapter_start_time.elapsed().as_secs_f32();

                if exit_status.success() {
                    println!(
                        "=========== PASSED! {} on {:?} ({} of {}) in {:.3}s ===========",
                        info.name, info.backend, idx, adapter_count, adapter_time
                    );
                } else {
                    println!(
                        "=========== FAILED! {} on {:?} ({} of {}) in {:.3}s ===========",
                        info.name, info.backend, idx, adapter_count, adapter_time
                    );
                    exit(1);
                }
            }

            let all_time = all_start.elapsed().as_secs_f32();

            println!(
                "=========== {} adapters PASSED in {:.3}s ===========",
                adapter_count, all_time
            );
        }
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    inner::main();
}
