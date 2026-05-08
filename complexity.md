## Summary Statistics

| Metric | Value |
|--------|-------|
| Files Analyzed | 14 |
| Total Functions | 107 |
| Total Lines | 1529 |
| Total Complexity | 198 |
| Avg Complexity / Function | 1.85 |
| Max Nesting Depth | 3 |
| Avg Nesting Depth | 0.50 |
| Avg Halstead Volume | 120.11 |
| Avg Halstead Difficulty | 3.87 |
| Avg Halstead Effort | 682.33 |
| Avg Halstead Time | 37.91 |

### ./src/analyzer.rs

#### File Summary

| Metric | Value |
|--------|-------|
| Total Functions | 10 |
| Total Lines | 120 |
| Total Function Lines | 97 |
| Total Complexity | 22 |
| Avg Complexity / Function | 2.20 |
| Max Complexity | 6 |
| Max Nesting Depth | 3 |
| Avg Nesting Depth | 0.80 |
| Max Function Lines | 27 |
| Avg Halstead Volume | 117.63 |
| Max Halstead Volume | 283.55 |
| Avg Halstead Difficulty | 5.01 |
| Max Halstead Difficulty | 10.25 |
| Avg Halstead Effort | 845.70 |
| Max Halstead Effort | 2906.44 |
| Avg Halstead Time | 46.98 |
| Max Halstead Time | 161.47 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| analyze_path | 14 | 15-28 | 4 | 1 | 213.62 | 6.82 | 1457.62 | 80.98 |
| analyze_directory | 10 | 30-39 | 4 | 3 | 151.27 | 5.65 | 855.24 | 47.51 |
| analyze_file | 15 | 41-55 | 6 | 3 | 252.01 | 9.25 | 2331.08 | 129.50 |
| max_u32 | 3 | 57-59 | 1 | 0 | 39.30 | 2.25 | 88.43 | 4.91 |
| max_usize | 3 | 61-63 | 1 | 0 | 39.30 | 2.25 | 88.43 | 4.91 |
| max_f64 | 3 | 65-67 | 1 | 0 | 47.55 | 2.00 | 95.10 | 5.28 |
| avg_f64 | 7 | 69-75 | 2 | 1 | 66.44 | 6.00 | 398.63 | 22.15 |
| build_success_result | 27 | 77-103 | 1 | 0 | 283.55 | 10.25 | 2906.44 | 161.47 |
| build_error_result | 8 | 105-112 | 1 | 0 | 48.43 | 2.86 | 138.38 | 7.69 |
| build_empty_result | 7 | 114-120 | 1 | 0 | 34.87 | 2.80 | 97.63 | 5.42 |

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
| Total Functions | 8 |
| Total Lines | 181 |
| Total Function Lines | 99 |
| Total Complexity | 11 |
| Avg Complexity / Function | 1.38 |
| Max Complexity | 2 |
| Max Nesting Depth | 1 |
| Avg Nesting Depth | 0.38 |
| Max Function Lines | 31 |
| Avg Halstead Volume | 81.91 |
| Max Halstead Volume | 330.22 |
| Avg Halstead Difficulty | 3.53 |
| Max Halstead Difficulty | 10.06 |
| Avg Halstead Effort | 534.82 |
| Max Halstead Effort | 3322.84 |
| Avg Halstead Time | 29.71 |
| Max Halstead Time | 184.60 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| default | 23 | 42-64 | 1 | 0 | 83.05 | 1.33 | 110.73 | 6.15 |
| default | 15 | 83-97 | 1 | 0 | 26.00 | 2.00 | 52.00 | 2.89 |
| sum_usize | 3 | 106-108 | 1 | 0 | 31.02 | 2.00 | 62.04 | 3.45 |
| sum_u32 | 3 | 110-112 | 1 | 0 | 31.02 | 2.00 | 62.04 | 3.45 |
| max_u32_from_files | 3 | 114-116 | 1 | 0 | 39.30 | 1.88 | 73.69 | 4.09 |
| safe_div | 7 | 118-124 | 2 | 1 | 30.88 | 3.00 | 92.64 | 5.15 |
| weighted_avg | 14 | 126-139 | 2 | 1 | 83.76 | 6.00 | 502.57 | 27.92 |
| from_results | 31 | 142-172 | 2 | 1 | 330.22 | 10.06 | 3322.84 | 184.60 |

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
| Total Lines | 123 |
| Total Function Lines | 93 |
| Total Complexity | 19 |
| Avg Complexity / Function | 3.17 |
| Max Complexity | 5 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 0.83 |
| Max Function Lines | 32 |
| Avg Halstead Volume | 190.31 |
| Max Halstead Volume | 380.55 |
| Avg Halstead Difficulty | 7.52 |
| Max Halstead Difficulty | 12.50 |
| Avg Halstead Effort | 1761.90 |
| Max Halstead Effort | 4756.90 |
| Avg Halstead Time | 97.88 |
| Max Halstead Time | 264.27 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| language_name | 3 | 20-22 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| analyze | 12 | 23-34 | 4 | 1 | 271.03 | 7.75 | 2100.47 | 116.69 |
| collect_functions | 15 | 38-52 | 3 | 1 | 140.65 | 6.50 | 914.21 | 50.79 |
| is_target_function | 9 | 54-62 | 5 | 1 | 114.71 | 9.60 | 1101.25 | 61.18 |
| build_function_complexity | 32 | 64-95 | 1 | 0 | 380.55 | 12.50 | 4756.90 | 264.27 |
| count_decisions | 22 | 97-118 | 5 | 2 | 216.84 | 7.75 | 1680.48 | 93.36 |

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
| Avg Halstead Volume | 90.65 |
| Max Halstead Volume | 227.43 |
| Avg Halstead Difficulty | 2.85 |
| Max Halstead Difficulty | 9.35 |
| Avg Halstead Effort | 363.14 |
| Max Halstead Effort | 2125.61 |
| Avg Halstead Time | 20.17 |
| Max Halstead Time | 118.09 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 32-34 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 36-38 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 40-45 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 47-58 | 1 | 0 | 43.02 | 1.00 | 43.02 | 2.39 |
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
| Avg Halstead Volume | 93.25 |
| Max Halstead Volume | 283.28 |
| Avg Halstead Difficulty | 2.83 |
| Max Halstead Difficulty | 10.33 |
| Avg Halstead Effort | 407.93 |
| Max Halstead Effort | 2927.18 |
| Avg Halstead Time | 22.66 |
| Max Halstead Time | 162.62 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 41-43 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 45-47 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 49-54 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 56-67 | 1 | 0 | 43.02 | 1.00 | 43.02 | 2.39 |
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
| Total Functions | 12 |
| Total Lines | 153 |
| Total Function Lines | 101 |
| Total Complexity | 21 |
| Avg Complexity / Function | 1.75 |
| Max Complexity | 6 |
| Max Nesting Depth | 2 |
| Avg Nesting Depth | 0.42 |
| Max Function Lines | 25 |
| Avg Halstead Volume | 105.80 |
| Max Halstead Volume | 285.40 |
| Avg Halstead Difficulty | 3.34 |
| Max Halstead Difficulty | 9.35 |
| Avg Halstead Effort | 545.12 |
| Max Halstead Effort | 2597.17 |
| Avg Halstead Time | 30.28 |
| Max Halstead Time | 144.29 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 30-32 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 34-36 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 38-43 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 45-56 | 1 | 0 | 38.05 | 1.00 | 38.05 | 2.11 |
| count_decisions_for_python | 25 | 59-83 | 6 | 2 | 285.40 | 9.10 | 2597.17 | 144.29 |
| extract_name | 12 | 85-96 | 4 | 2 | 227.43 | 9.35 | 2125.61 | 118.09 |
| test_simple_function | 8 | 103-110 | 1 | 0 | 134.89 | 2.36 | 317.95 | 17.66 |
| test_if_elif_else | 6 | 113-118 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_match | 6 | 121-126 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |
| test_lambda_included | 8 | 129-136 | 1 | 0 | 148.68 | 2.16 | 320.59 | 17.81 |
| test_lambda_excluded_by_default | 6 | 139-144 | 1 | 0 | 68.11 | 2.00 | 136.23 | 7.57 |
| test_try_except | 6 | 147-152 | 1 | 0 | 74.01 | 1.95 | 144.32 | 8.02 |

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
| Avg Halstead Volume | 83.02 |
| Max Halstead Volume | 205.13 |
| Avg Halstead Difficulty | 3.01 |
| Max Halstead Difficulty | 9.33 |
| Avg Halstead Effort | 350.75 |
| Max Halstead Effort | 1775.20 |
| Avg Halstead Time | 19.49 |
| Max Halstead Time | 98.62 |

| Function | Lines | Line Range | Complexity | Nesting | Halstead Vol | Difficulty | Halstead Effort | Halstead Time |
|----------|-------|------------|------------|---------|--------------|------------|-----------------|---------------|
| can_analyze | 3 | 29-31 | 1 | 0 | 30.88 | 2.25 | 69.48 | 3.86 |
| language_name | 3 | 33-35 | 1 | 0 | 18.09 | 1.00 | 18.09 | 1.01 |
| parser | 6 | 37-42 | 2 | 1 | 96.00 | 5.06 | 485.33 | 26.96 |
| config | 12 | 44-55 | 1 | 0 | 41.51 | 1.00 | 41.51 | 2.31 |
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


