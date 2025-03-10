use super::current_params::*;

use super::{
    ClassicPBSParameters, CompactPublicKeyEncryptionParameters, CompressionParameters,
    MultiBitPBSParameters, ShortintKeySwitchingParameters,
};

// Classic
// CPK Gaussian
pub const TEST_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_5_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_5_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_5_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_6_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_6_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_7_CARRY_1_COMPACT_PK_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_PBS_KS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_PBS_KS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_4_COMPACT_PK_PBS_KS_GAUSSIAN_2M128;

// KS PBS Gaussian
// 2^-64
pub const TEST_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M64: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_4_KS_PBS_GAUSSIAN_2M64;

// 2^-128
pub const TEST_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_2_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_3_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_4_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_5_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_6_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_1_CARRY_7_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_1_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_3_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_4_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_5_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_6_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_1_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_2_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_4_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_3_CARRY_5_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_1_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_2_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_4_CARRY_3_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_5_CARRY_1_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_5_CARRY_2_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_5_CARRY_3_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_6_CARRY_1_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_6_CARRY_2_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_7_CARRY_1_KS_PBS_GAUSSIAN_2M128;

// PBS KS Gaussian
pub const TEST_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_2_PBS_KS_GAUSSIAN_2M128;

// KS PBS TUniform
pub const TEST_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: ClassicPBSParameters =
    V1_1_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;

// MultiBit
// CPU Gaussian
pub const TEST_PARAM_MULTI_BIT_GROUP_3_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128:
    MultiBitPBSParameters = V1_1_PARAM_MULTI_BIT_GROUP_3_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M128;
pub const TEST_PARAM_MULTI_BIT_GROUP_2_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_MULTI_BIT_GROUP_2_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MULTI_BIT_GROUP_2_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_MULTI_BIT_GROUP_2_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MULTI_BIT_GROUP_2_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_MULTI_BIT_GROUP_2_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MULTI_BIT_GROUP_3_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_MULTI_BIT_GROUP_3_MESSAGE_1_CARRY_1_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MULTI_BIT_GROUP_3_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_MULTI_BIT_GROUP_3_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_MULTI_BIT_GROUP_3_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_MULTI_BIT_GROUP_3_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64;
// GPU Gaussian
pub const TEST_PARAM_GPU_MULTI_BIT_GROUP_3_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_GPU_MULTI_BIT_GROUP_3_MESSAGE_2_CARRY_2_KS_PBS_GAUSSIAN_2M64;
pub const TEST_PARAM_GPU_MULTI_BIT_GROUP_3_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64:
    MultiBitPBSParameters = V1_1_PARAM_GPU_MULTI_BIT_GROUP_3_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M64;

// GPU TUniform
pub const TEST_PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128:
    MultiBitPBSParameters =
    V1_1_PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;

// PKE
pub const TEST_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128:
    CompactPublicKeyEncryptionParameters = V1_1_PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
pub const TEST_PARAM_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1:
    CompactPublicKeyEncryptionParameters =
    V1_1_PARAM_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1;
pub const TEST_PARAM_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV2:
    CompactPublicKeyEncryptionParameters =
    V1_1_PARAM_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV2;

// KS
pub const TEST_PARAM_KEYSWITCH_1_1_KS_PBS_TO_2_2_KS_PBS_GAUSSIAN_2M128:
    ShortintKeySwitchingParameters = V1_1_PARAM_KEYSWITCH_1_1_KS_PBS_TO_2_2_KS_PBS_GAUSSIAN_2M128;

pub const TEST_PARAM_KEYSWITCH_PKE_TO_SMALL_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128:
    ShortintKeySwitchingParameters =
    V1_1_PARAM_KEYSWITCH_PKE_TO_SMALL_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
pub const TEST_PARAM_KEYSWITCH_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128:
    ShortintKeySwitchingParameters =
    V1_1_PARAM_KEYSWITCH_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
// ZKV1
pub const TEST_PARAM_KEYSWITCH_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1:
    ShortintKeySwitchingParameters =
    V1_1_PARAM_KEYSWITCH_PKE_TO_BIG_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128_ZKV1;

// Compression
pub const TEST_COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128: CompressionParameters =
    V1_1_COMP_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
pub const TEST_COMP_PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128:
    CompressionParameters =
    V1_1_COMP_PARAM_GPU_MULTI_BIT_GROUP_4_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
