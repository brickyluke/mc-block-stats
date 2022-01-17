# Minecraft Block Statistics Analyser

Counts Minecraft blocks by type and altitude (Y coordinate, height) for a given set of Minecraft region files (`*.mca`).

Prints resulting statistics in CSV format.

## Build

Use standard Cargo commands to build and install the executable.

```bash
cargo build

cargo install --path .
```

## Usage

```
USAGE:
    mc-block-stats [FLAGS] [OPTIONS] <FILE>...

FLAGS:
    -a, --all-chunks     Process all chunks, including those that haven't been fully populated yet
        --help           Prints help information
    -h, --high-worlds    Expect high worlds; use this for Minecraft 1.18 and later: -64 <= y < 320
    -q, --quiet          Silence all output
    -V, --version        Prints version information
    -v, --verbose        Verbose mode (-v, -vv, -vvv, etc)

OPTIONS:
    -t, --threads <threads>    Number of concurrent threads; defaults to the number of available CPU cores

ARGS:
    <FILE>...    Minecraft region files (*.mca)
```

### Example

```bash
mc-block-stats -a -h -v mc118-world/region/*.mca
```

Sample output:
```csv
block_type,y_-64,y_-63,y_-62,y_-61,y_-60,y_-59,y_-58,y_-57,y_-56,y_-55,y_-54,y_-53,y_-52,y_-51,y_-50,y_-49,y_-48,y_-47,y_-46,y_-45,y_-44,y_-43,y_-42,y_-41,y_-40,y_-39,y_-38,y_-37,y_-36,y_-35,y_-34,y_-33,y_-32,y_-31,y_-30,y_-29,y_-28,y_-27,y_-26,y_-25,y_-24,y_-23,y_-22,y_-21,y_-20,y_-19,y_-18,y_-17,y_-16,y_-15,y_-14,y_-13,y_-12,y_-11,y_-10,y_-9,y_-8,y_-7,y_-6,y_-5,y_-4,y_-3,y_-2,y_-1,y_0,y_1,y_2,y_3,y_4,y_5,y_6,y_7,y_8,y_9,y_10,y_11,y_12,y_13,y_14,y_15,y_16,y_17,y_18,y_19,y_20,y_21,y_22,y_23,y_24,y_25,y_26,y_27,y_28,y_29,y_30,y_31,y_32,y_33,y_34,y_35,y_36,y_37,y_38,y_39,y_40,y_41,y_42,y_43,y_44,y_45,y_46,y_47,y_48,y_49,y_50,y_51,y_52,y_53,y_54,y_55,y_56,y_57,y_58,y_59,y_60,y_61,y_62,y_63,y_64,y_65,y_66,y_67,y_68,y_69,y_70,y_71,y_72,y_73,y_74,y_75,y_76,y_77,y_78,y_79,y_80,y_81,y_82,y_83,y_84,y_85,y_86,y_87,y_88,y_89,y_90,y_91,y_92,y_93,y_94,y_95,y_96,y_97,y_98,y_99,y_100,y_101,y_102,y_103,y_104,y_105,y_106,y_107,y_108,y_109,y_110,y_111,y_112,y_113,y_114,y_115,y_116,y_117,y_118,y_119,y_120,y_121,y_122,y_123,y_124,y_125,y_126,y_127,y_128,y_129,y_130,y_131,y_132,y_133,y_134,y_135,y_136,y_137,y_138,y_139,y_140,y_141,y_142,y_143,y_144,y_145,y_146,y_147,y_148,y_149,y_150,y_151,y_152,y_153,y_154,y_155,y_156,y_157,y_158,y_159,y_160,y_161,y_162,y_163,y_164,y_165,y_166,y_167,y_168,y_169,y_170,y_171,y_172,y_173,y_174,y_175,y_176,y_177,y_178,y_179,...,y_319
"minecraft:amethyst_block",0,0,0,0,18,146,388,534,720,878,991,1295,1548,1695,1809,2058,2176,2222,2272,2288,2212,2146,2159,2035,1918,2053,2055,1988,2108,2170,2195,2291,2414,2425,2415,2562,2538,2648,2581,2401,2427,2416,2350,2429,2461,2450,2492,2591,2710,2712,2736,2725,2725,2753,2665,2680,2620,2423,2319,2562,2684,2589,2522,2473,2541,2606,2675,2666,2657,2738,2670,2452,2157,2134,2277,2448,2369,2386,2423,2340,2290,2273,2295,2265,2360,2392,2313,2141,2056,2045,2089,1997,1927,1970,1757,1551,1348,1121,1038,916,738,565,465,347,250,134,21,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,...,0
"minecraft:amethyst_cluster",0,0,0,0,0,0,0,0,0,3,7,5,7,6,10,10,9,11,12,4,6,11,11,14,12,10,6,13,8,5,9,10,9,12,6,10,8,10,5,15,8,10,11,15,15,15,12,11,16,19,32,13,21,8,13,15,16,12,12,11,11,13,16,12,10,9,15,13,19,6,8,12,8,9,10,9,14,19,12,5,10,11,7,9,12,8,12,8,12,14,8,7,13,9,9,7,9,9,9,2,4,4,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,...,0
"minecraft:andesite",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1826,13427,38087,73353,114219,154390,188432,214485,227318,229694,230216,228493,226280,224657,222687,224349,225297,224495,223906,224102,223307,221746,218967,221242,224057,225558,224262,222198,221818,222555,222052,224797,228711,231834,233654,234339,233863,231558,228431,230385,231532,231713,229912,226539,221718,216303,211127,209780,207406,206344,204051,199942,186387,165120,156911,151150,143707,136860,128220,122244,117652,112676,102128,85672,65192,43940,25845,12496,4713,1814,1467,1485,1497,1345,1074,893,915,946,1076,1175,1182,1117,948,773,614,415,286,187,132,119,221,300,298,300,294,279,254,232,208,173,125,67,53,72,154,199,215,195,140,89,14,0,0,0,0,0,0,0,0,0,1,8,13,11,8,8,10,11,8,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,...,0
"minecraft:azalea",0,0,0,0,0,0,0,0,0,0,1,5,23,36,25,21,25,24,47,59,72,95,68,93,93,36,46,48,49,51,52,58,59,62,68,99,92,87,102,122,156,105,123,130,123,181,204,219,328,174,167,203,183,210,205,250,268,141,193,181,190,212,203,229,301,153,148,160,173,211,187,209,249,113,112,113,147,147,165,124,152,98,98,85,79,81,88,68,82,29,46,50,33,46,46,22,22,24,29,34,29,24,28,16,22,14,9,6,8,4,4,22,7,9,0,4,4,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,...,0
"minecraft:bedrock",3357696,2687254,2014355,1341861,670201,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,...,0
...
"minecraft:water",0,0,0,0,0,0,0,0,0,43,3547,1715,2042,2430,2246,2596,3392,4230,4797,5443,6772,8924,10390,12066,15166,15937,16763,18702,20622,22827,24382,25612,26841,26450,26742,26949,27876,29584,27331,31430,36456,17972,18009,18724,7296,7969,8947,7211,8747,8755,9757,11117,11781,12764,13703,15072,16848,17161,19696,24364,32677,45621,61204,82564,108097,119156,130256,140336,148177,157091,165796,178734,197651,192202,194112,200055,204673,209354,199946,208167,221558,152185,147371,144557,97010,103155,114068,123309,140740,142410,144307,146782,145402,145106,145781,149348,157410,143763,157626,185098,230308,292191,366638,438682,513002,531801,551700,567926,591109,618680,678575,811738,1041488,1140797,1227200,1300471,1369640,1436324,1510116,1592251,1673916,1726269,1772888,1818446,1867497,1928878,2046960,231,201,124,96,104,97,87,119,133,98,88,81,75,79,67,66,62,64,46,52,46,55,49,45,41,34,31,30,33,32,32,31,28,41,32,29,26,26,22,22,28,27,27,26,23,20,22,22,22,19,21,25,20,18,14,15,14,15,14,15,15,14,14,14,14,14,14,15,14,12,12,13,12,12,23,23,21,22,20,12,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,6,6,6,4,4,2,3,1,1,2,...,0
```
