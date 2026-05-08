## Summary Statistics

| Metric | Value |
|--------|-------|
| Files Analyzed | 14 |
| Total Functions | 102 |
| Total Lines | 1549 |
| Total Complexity | 193 |
| Avg Complexity / Function | 1.89 |
| Max Nesting Depth | 3 |
| Avg Nesting Depth | 0.52 |
| Avg Halstead Volume | 129.42 |
| Avg Halstead Difficulty | 4.00 |
| Avg Halstead Effort | 874.71 |
| Avg Halstead Time | 48.60 |

### ./src/analyzer.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 5 |
| Total Lines | 80 |
| Total Function Lines | 62 |
| Total Complexity | 18 |
| Avg Complexity / Function | 3.60 |
| Max Complexity | 7 |
| Max Nesting Depth | 3 |
| Avg Nesting Depth | 1.60 |
| Max Function Lines | 18 |
| Avg Halstead Volume | 161.74 |
| Max Halstead Volume | 325.48 |
| Avg Halstead Difficulty | 5.71 |
| Max Halstead Difficulty | 9.14 |
| Avg Halstead Effort | 1211.09 |
| Max Halstead Effort | 2975.86 |
| Avg Halstead Time | 67.28 |
| Max Halstead Time | 165.33 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| analyze_path | 12 | 15-26 | 3 | 2 | 176.42 | 4.94 | 871.72 | 48.43 |
| analyze_directory | 17 | 28-44 | 6 | 3 | 223.48 | 8.82 | 1971.88 | 109.55 |
| analyze_file | 18 | 46-63 | 7 | 3 | 325.48 | 9.14 | 2975.86 | 165.33 |
| build_error_result | 8 | 65-72 | 1 | 0 | 48.43 | 2.86 | 138.38 | 7.69 |
| build_empty_result | 7 | 74-80 | 1 | 0 | 34.87 | 2.80 | 97.63 | 5.42 |

### ./src/complexity.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 3 |
| Total Lines | 43 |
| Total Function Lines | 33 |
| Total Complexity | 9 |
| Avg Complexity / Function | 3.00 |
| Max Complexity | 4 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 1.67 |
| Max Function Lines | 13 |
| Avg Halstead Volume | 109.91 |
| Max Halstead Volume | 164.09 |
| Avg Halstead Difficulty | 5.92 |
| Max Halstead Difficulty | 7.67 |
| Avg Halstead Effort | 693.76 |
| Max Halstead Effort | 1258.02 |
| Avg Halstead Time | 38.54 |
| Max Halstead Time | 69.89 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| count_descendants_of_kind | 13 | 5-17 | 4 | 2 | 164.09 | 7.67 | 1258.02 | 69.89 |
| count_children_of_kind | 10 | 20-29 | 3 | 2 | 99.91 | 4.69 | 468.34 | 26.02 |
| is_boolean_operator | 10 | 34-43 | 2 | 1 | 65.73 | 5.40 | 354.94 | 19.72 |

### ./src/lib.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 9 |
| Total Lines | 250 |
| Total Function Lines | 164 |
| Total Complexity | 14 |
| Avg Complexity / Function | 1.56 |
| Max Complexity | 3 |
| Max Nesting Depth | 1 |
| Avg Nesting Depth | 0.44 |
| Max Function Lines | 65 |
| Avg Halstead Volume | 171.21 |
| Max Halstead Volume | 885.61 |
| Avg Halstead Difficulty | 5.56 |
| Max Halstead Difficulty | 21.79 |
| Avg Halstead Effort | 2619.40 |
| Max Halstead Effort | 19296.02 |
| Avg Halstead Time | 145.52 |
| Max Halstead Time | 1072.00 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| default | 23 | 43-65 | 1 | 0 | 83.05 | 1.33 | 110.73 | 6.15 |
| from_functions | 65 | 69-133 | 3 | 1 | 885.61 | 21.79 | 19296.02 | 1072.00 |
| default | 15 | 152-166 | 1 | 0 | 26.00 | 2.00 | 52.00 | 2.89 |
| sum_usize | 3 | 175-177 | 1 | 0 | 31.02 | 2.00 | 62.04 | 3.45 |
| sum_u32 | 3 | 179-181 | 1 | 0 | 31.02 | 2.00 | 62.04 | 3.45 |
| max_u32_from_files | 3 | 183-185 | 1 | 0 | 39.30 | 1.88 | 73.69 | 4.09 |
| safe_div | 7 | 187-193 | 2 | 1 | 30.88 | 3.00 | 92.64 | 5.15 |
| weighted_avg | 14 | 195-208 | 2 | 1 | 83.76 | 6.00 | 502.57 | 27.92 |
| from_results | 31 | 211-241 | 2 | 1 | 330.22 | 10.06 | 3322.84 | 184.60 |

### ./src/output/json.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 1 |
| Total Lines | 16 |
| Total Function Lines | 10 |
| Total Complexity | 1 |
| Avg Complexity / Function | 1.00 |
| Max Complexity | 1 |
| Max Nesting Depth | 0 |
| Avg Nesting Depth | 0.00 |
| Max Function Lines | 10 |
| Avg Halstead Volume | 77.71 |
| Max Halstead Volume | 77.71 |
| Avg Halstead Difficulty | 2.89 |
| Max Halstead Difficulty | 2.89 |
| Avg Halstead Effort | 224.49 |
| Max Halstead Effort | 224.49 |
| Avg Halstead Time | 12.47 |
| Max Halstead Time | 12.47 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| format | 10 | 6-15 | 1 | 0 | 77.71 | 2.89 | 224.49 | 12.47 |

### ./src/output/mod.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 1 |
| Total Lines | 17 |
| Total Function Lines | 7 |
| Total Complexity | 4 |
| Avg Complexity / Function | 4.00 |
| Max Complexity | 4 |
| Max Nesting Depth | 1 |
| Avg Nesting Depth | 1.00 |
| Max Function Lines | 7 |
| Avg Halstead Volume | 98.10 |
| Max Halstead Volume | 98.10 |
| Avg Halstead Difficulty | 3.54 |
| Max Halstead Difficulty | 3.54 |
| Avg Halstead Effort | 347.43 |
| Max Halstead Effort | 347.43 |
| Avg Halstead Time | 19.30 |
| Max Halstead Time | 19.30 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| get_formatter | 7 | 11-17 | 4 | 1 | 98.10 | 3.54 | 347.43 | 19.30 |

### ./src/output/markdown.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 8 |
| Total Lines | 132 |
| Total Function Lines | 119 |
| Total Complexity | 16 |
| Avg Complexity / Function | 2.00 |
| Max Complexity | 3 |
| Max Nesting Depth | 1 |
| Avg Nesting Depth | 0.62 |
| Max Function Lines | 35 |
| Avg Halstead Volume | 241.51 |
| Max Halstead Volume | 758.28 |
| Avg Halstead Difficulty | 4.17 |
| Max Halstead Difficulty | 7.00 |
| Avg Halstead Effort | 1308.22 |
| Max Halstead Effort | 5307.93 |
| Avg Halstead Time | 72.68 |
| Max Halstead Time | 294.88 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| format | 14 | 6-19 | 3 | 1 | 137.61 | 4.04 | 555.72 | 30.87 |
| format_summary | 24 | 22-45 | 2 | 1 | 485.31 | 4.66 | 2262.59 | 125.70 |
| format_file | 14 | 47-60 | 3 | 1 | 224.01 | 5.80 | 1299.25 | 72.18 |
| format_file_summary | 35 | 62-96 | 3 | 1 | 758.28 | 7.00 | 5307.93 | 294.88 |
| format_function_table | 11 | 98-108 | 2 | 1 | 87.57 | 3.20 | 280.22 | 15.57 |
| metric_row | 3 | 110-112 | 1 | 0 | 50.19 | 3.21 | 161.32 | 8.96 |
| metric_row_f64 | 3 | 114-116 | 1 | 0 | 34.87 | 2.00 | 69.74 | 3.87 |
| format_function_row | 15 | 118-132 | 1 | 0 | 154.29 | 3.43 | 528.98 | 29.39 |

### ./src/output/pretty.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 2 |
| Total Lines | 44 |
| Total Function Lines | 36 |
| Total Complexity | 5 |
| Avg Complexity / Function | 2.50 |
| Max Complexity | 4 |
| Max Nesting Depth | 1 |
| Avg Nesting Depth | 0.50 |
| Max Function Lines | 33 |
| Avg Halstead Volume | 308.78 |
| Max Halstead Volume | 586.68 |
| Avg Halstead Difficulty | 3.89 |
| Max Halstead Difficulty | 5.91 |
| Avg Halstead Effort | 1762.32 |
| Max Halstead Effort | 3466.74 |
| Avg Halstead Time | 97.91 |
| Max Halstead Time | 192.60 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| format | 3 | 7-9 | 1 | 0 | 30.88 | 1.88 | 57.90 | 3.22 |
| format_file_entry | 33 | 12-44 | 4 | 1 | 586.68 | 5.91 | 3466.74 | 192.60 |

### ./src/language/mod.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 6 |
| Total Lines | 140 |
| Total Function Lines | 104 |
| Total Complexity | 21 |
| Avg Complexity / Function | 3.50 |
| Max Complexity | 7 |
| Max Nesting Depth | 3 |
| Avg Nesting Depth | 1.00 |
| Max Function Lines | 37 |
| Avg Halstead Volume | 211.55 |
| Max Halstead Volume | 390.08 |
| Avg Halstead Difficulty | 7.98 |
| Max Halstead Difficulty | 12.00 |
| Avg Halstead Effort | 2082.79 |
| Max Halstead Effort | 4681.01 |
| Avg Halstead Time | 115.71 |
| Max Halstead Time | 260.06 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| language_name | 3 | 26-28 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| analyze | 12 | 29-40 | 4 | 1 | 271.03 | 7.75 | 2100.47 | 116.69 |
| collect_functions | 15 | 44-58 | 3 | 1 | 140.65 | 6.50 | 914.21 | 50.79 |
| is_target_function | 9 | 60-68 | 5 | 1 | 114.71 | 9.60 | 1101.25 | 61.18 |
| build_function_complexity | 37 | 70-106 | 1 | 0 | 390.08 | 12.00 | 4681.01 | 260.06 |
| count_decisions | 28 | 108-135 | 7 | 3 | 334.70 | 11.00 | 3681.72 | 204.54 |

### ./src/language/rust.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 11 |
| Total Lines | 144 |
| Total Function Lines | 91 |
| Total Complexity | 15 |
| Avg Complexity / Function | 1.36 |
| Max Complexity | 4 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 0.27 |
| Max Function Lines | 14 |
| Avg Halstead Volume | 90.06 |
| Max Halstead Volume | 227.43 |
| Avg Halstead Difficulty | 2.85 |
| Max Halstead Difficulty | 9.35 |
| Avg Halstead Effort | 362.55 |
| Max Halstead Effort | 2125.61 |
| Avg Halstead Time | 20.14 |
| Max Halstead Time | 118.09 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 32-34 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 36-38 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 40-45 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 47-58 | 1 | 0 | 36.54 | 1.00 | 36.54 | 2.03 |
| extract_name | 12 | 61-72 | 4 | 2 | 227.43 | 9.35 | 2125.61 | 118.09 |
| test_simple_function | 8 | 79-86 | 1 | 0 | 134.89 | 2.36 | 317.95 | 17.66 |
| test_if_else_if | 12 | 89-100 | 1 | 0 | 68.11 | 2.00 | 136.23 | 7.57 |
| test_match | 14 | 103-116 | 1 | 0 | 68.11 | 2.00 | 136.23 | 7.57 |
| test_closure_included | 8 | 119-126 | 1 | 0 | 118.03 | 2.45 | 289.71 | 16.09 |
| test_closure_excluded_by_default | 7 | 129-135 | 1 | 0 | 118.54 | 1.93 | 228.61 | 12.70 |
| test_boolean_ops | 6 | 138-143 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |

### ./src/language/javascript.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 12 |
| Total Lines | 146 |
| Total Function Lines | 82 |
| Total Complexity | 18 |
| Avg Complexity / Function | 1.50 |
| Max Complexity | 6 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 0.25 |
| Max Function Lines | 12 |
| Avg Halstead Volume | 92.71 |
| Max Halstead Volume | 283.28 |
| Avg Halstead Difficulty | 2.83 |
| Max Halstead Difficulty | 10.33 |
| Avg Halstead Effort | 407.39 |
| Max Halstead Effort | 2927.18 |
| Avg Halstead Time | 22.63 |
| Max Halstead Time | 162.62 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 41-43 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 45-47 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 49-54 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 56-67 | 1 | 0 | 36.54 | 1.00 | 36.54 | 2.03 |
| extract_name | 12 | 70-81 | 6 | 2 | 283.28 | 10.33 | 2927.18 | 162.62 |
| test_simple_function | 8 | 88-95 | 1 | 0 | 134.89 | 2.36 | 317.95 | 17.66 |
| test_if_else | 6 | 98-103 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_switch | 6 | 106-111 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_arrow_function_included | 8 | 114-121 | 1 | 0 | 148.68 | 2.16 | 320.59 | 17.81 |
| test_arrow_function_excluded_by_default | 6 | 124-129 | 1 | 0 | 68.11 | 2.00 | 136.23 | 7.57 |
| test_try_catch | 6 | 132-137 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_boolean_ops | 6 | 140-145 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |

### ./src/language/python.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 11 |
| Total Lines | 127 |
| Total Function Lines | 76 |
| Total Complexity | 15 |
| Avg Complexity / Function | 1.36 |
| Max Complexity | 4 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 0.27 |
| Max Function Lines | 12 |
| Avg Halstead Volume | 90.25 |
| Max Halstead Volume | 227.43 |
| Avg Halstead Difficulty | 2.82 |
| Max Halstead Difficulty | 9.35 |
| Avg Halstead Effort | 359.35 |
| Max Halstead Effort | 2125.61 |
| Avg Halstead Time | 19.96 |
| Max Halstead Time | 118.09 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 30-32 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 34-36 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 38-43 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 45-56 | 1 | 0 | 46.60 | 1.00 | 46.60 | 2.59 |
| extract_name | 12 | 59-70 | 4 | 2 | 227.43 | 9.35 | 2125.61 | 118.09 |
| test_simple_function | 8 | 77-84 | 1 | 0 | 134.89 | 2.36 | 317.95 | 17.66 |
| test_if_elif_else | 6 | 87-92 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_match | 6 | 95-100 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_lambda_included | 8 | 103-110 | 1 | 0 | 148.68 | 2.16 | 320.59 | 17.81 |
| test_lambda_excluded_by_default | 6 | 113-118 | 1 | 0 | 68.11 | 2.00 | 136.23 | 7.57 |
| test_try_except | 6 | 121-126 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |

### ./src/language/c.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 15 |
| Total Lines | 176 |
| Total Function Lines | 119 |
| Total Complexity | 22 |
| Avg Complexity / Function | 1.47 |
| Max Complexity | 4 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 0.33 |
| Max Function Lines | 14 |
| Avg Halstead Volume | 82.58 |
| Max Halstead Volume | 205.13 |
| Avg Halstead Difficulty | 3.01 |
| Max Halstead Difficulty | 9.33 |
| Avg Halstead Effort | 350.31 |
| Max Halstead Effort | 1775.20 |
| Avg Halstead Time | 19.46 |
| Max Halstead Time | 98.62 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 29-31 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 33-35 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 37-42 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 44-55 | 1 | 0 | 34.87 | 1.00 | 34.87 | 1.94 |
| extract_name | 14 | 58-71 | 4 | 2 | 205.13 | 8.65 | 1775.20 | 98.62 |
| find_identifier_in_declarator | 13 | 73-85 | 4 | 2 | 155.32 | 9.33 | 1449.69 | 80.54 |
| test_simple_function | 8 | 92-99 | 1 | 0 | 134.89 | 2.36 | 317.95 | 17.66 |
| test_if_else | 6 | 102-107 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_switch | 14 | 110-123 | 1 | 0 | 68.11 | 2.00 | 136.23 | 7.57 |
| test_for_while_do | 12 | 126-137 | 1 | 0 | 68.11 | 2.00 | 136.23 | 7.57 |
| test_ternary | 6 | 140-145 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_boolean_ops | 6 | 148-153 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_pointer_return_name | 7 | 156-162 | 1 | 0 | 101.58 | 2.12 | 215.86 | 11.99 |
| test_parse_error | 5 | 165-169 | 1 | 0 | 51.89 | 1.88 | 97.30 | 5.41 |
| test_can_analyze_header | 4 | 172-175 | 1 | 0 | 51.81 | 1.65 | 85.48 | 4.75 |

### ./src/cognitive.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 17 |
| Total Lines | 195 |
| Total Function Lines | 157 |
| Total Complexity | 30 |
| Avg Complexity / Function | 1.76 |
| Max Complexity | 5 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 0.41 |
| Max Function Lines | 29 |
| Avg Halstead Volume | 111.61 |
| Max Halstead Volume | 319.63 |
| Avg Halstead Difficulty | 4.21 |
| Max Halstead Difficulty | 10.22 |
| Avg Halstead Effort | 664.76 |
| Max Halstead Effort | 2355.17 |
| Avg Halstead Time | 36.93 |
| Max Halstead Time | 130.84 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| max_nesting_depth | 3 | 4-6 | 1 | 0 | 42.00 | 1.50 | 63.00 | 3.50 |
| compute_nesting_depth | 16 | 8-23 | 5 | 2 | 237.88 | 9.62 | 2289.63 | 127.20 |
| new | 8 | 33-40 | 1 | 0 | 16.00 | 1.17 | 18.67 | 1.04 |
| add_operator | 4 | 42-45 | 1 | 0 | 39.30 | 2.62 | 103.17 | 5.73 |
| add_operand | 4 | 47-50 | 1 | 0 | 39.30 | 2.62 | 103.17 | 5.73 |
| n1 | 3 | 52-54 | 1 | 0 | 16.25 | 2.25 | 36.57 | 2.03 |
| n2 | 3 | 56-58 | 1 | 0 | 16.25 | 2.25 | 36.57 | 2.03 |
| n | 3 | 60-62 | 1 | 0 | 23.26 | 4.00 | 93.06 | 5.17 |
| total | 3 | 64-66 | 1 | 0 | 23.26 | 4.00 | 93.06 | 5.17 |
| halstead_metrics | 29 | 69-97 | 3 | 1 | 319.63 | 7.37 | 2355.17 | 130.84 |
| collect_halstead | 24 | 99-122 | 5 | 2 | 256.00 | 7.95 | 2036.36 | 113.13 |
| parse_rust | 6 | 129-134 | 1 | 0 | 95.18 | 3.00 | 285.55 | 15.86 |
| find_function | 12 | 136-147 | 4 | 2 | 171.67 | 10.22 | 1754.88 | 97.49 |
| test_nesting_depth_simple | 8 | 150-157 | 1 | 0 | 131.69 | 2.80 | 368.72 | 20.48 |
| test_nesting_depth_nested | 8 | 160-167 | 1 | 0 | 138.30 | 2.75 | 380.33 | 21.13 |
| test_nesting_depth_skips_inner_function | 8 | 170-177 | 1 | 0 | 131.69 | 2.80 | 368.72 | 20.48 |
| test_halstead_basic | 15 | 180-194 | 1 | 0 | 199.69 | 4.58 | 914.35 | 50.80 |

### ./src/main.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 1 |
| Total Lines | 39 |
| Total Function Lines | 20 |
| Total Complexity | 5 |
| Avg Complexity / Function | 5.00 |
| Max Complexity | 5 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 2.00 |
| Max Function Lines | 20 |
| Avg Halstead Volume | 295.00 |
| Max Halstead Volume | 295.00 |
| Avg Halstead Difficulty | 3.61 |
| Max Halstead Difficulty | 3.61 |
| Avg Halstead Effort | 1065.28 |
| Max Halstead Effort | 1065.28 |
| Avg Halstead Time | 59.18 |
| Max Halstead Time | 59.18 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| main | 20 | 20-39 | 5 | 2 | 295.00 | 3.61 | 1065.28 | 59.18 |


