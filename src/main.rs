// src/main.rs
// GPU Inference Proxy - Foundation

use k8s_resource_spy::gpu_metrics::{GpuMetrics, AppleSiliconGpu};

fn main() {
    println!("--- k8s-resource-spy: GPU Monitoring ---\n");

    // Single GPU test
    let gpu = AppleSiliconGpu::new(0);
    print_gpu_info(&gpu);
    
    // TODO: add multiple GPU cluster simulation
    // TODO: implement least-loaded GPU selection for routing
}

/// Print GPU metrics in formatted output
fn print_gpu_info(gpu: &AppleSiliconGpu) {
    println!("GPU: {}", gpu.name());
    println!("  Utilization: {:.1}%", gpu.utilization());
    println!("  Memory: {:.2} / {:.2} GB",
        gpu.memory_used() as f64 / 1024.0 / 1024.0 / 1024.0,
        gpu.memory_total() as f64 / 1024.0 / 1024.0 / 1024.0);
    println!("  Temperature: {:.1}°C", gpu.temperature());
}
