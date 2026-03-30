// src/gpu_metrics.rs
// GPU 메트릭을 추상화하는 Trait 정의

/// GPU 장치가 제공해야 하는 공통 인터페이스
/// 
/// 이 Trait을 구현하면:
/// - Apple Silicon GPU (M1/M2/M3)
/// - NVIDIA GPU (A100, H100 등)
/// - AMD GPU
/// 모두 동일한 방식으로 사용 가능
pub trait GpuMetrics {
    /// GPU 사용률 반환 (0.0 ~ 100.0)
    fn utilization(&self) -> f32;
    
    /// 현재 사용 중인 메모리 (bytes)
    fn memory_used(&self) -> u64;
    
    /// 전체 메모리 용량 (bytes)
    fn memory_total(&self) -> u64;
    
    /// GPU 온도 (섭씨)
    fn temperature(&self) -> f32;
    
    /// GPU 이름
    fn name(&self) -> String;
}

/// Apple Silicon (M3) GPU 구현체
/// 
/// M3 맥북에서 Mock 데이터로 개발하기 위한 구조체
pub struct AppleSiliconGpu {
    gpu_id: u32,
}

impl AppleSiliconGpu {
    /// 새로운 M3 GPU 인스턴스 생성
    pub fn new(gpu_id: u32) -> Self {
        Self { gpu_id }
    }
}

impl GpuMetrics for AppleSiliconGpu {
    fn utilization(&self) -> f32 {
        // TODO: implement real M3 GPU utilization via IOKit or Activity Monitor API
        // Currently using mock data for development
        45.0
    }
    
    fn memory_used(&self) -> u64 {
        // TODO: query actual M3 unified memory usage
        // Mock: 2GB
        2 * 1024 * 1024 * 1024
    }
    
    fn memory_total(&self) -> u64 {
        // TODO: detect M3 Pro/Max/Ultra variant and return actual memory
        // Mock: 18GB (M3 Pro)
        18 * 1024 * 1024 * 1024
    }
    
    fn temperature(&self) -> f32 {
        // TODO: read M3 GPU temperature via SMC (System Management Controller)
        // Mock: 55°C
        55.0
    }
    
    fn name(&self) -> String {
        format!("Apple M3 GPU #{}", self.gpu_id)
    }
}

// ============================================
// 나중에 구현할 NVIDIA GPU (Week 4-5)
// ============================================

/// NVIDIA GPU 구현체
/// 
/// AWS나 Colab에서 테스트할 때 사용
pub struct NvidiaGpu {
    #[allow(dead_code)]  // Week 4-5에 사용 예정
    device_id: u32,
}

impl NvidiaGpu {
    pub fn new(device_id: u32) -> Self {
        Self { device_id }
    }
}

// TODO: implement GpuMetrics for NvidiaGpu
// Planned: Week 4-5, requires nvml-wrapper crate and NVIDIA GPU hardware
