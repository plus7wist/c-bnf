import sys
import case_convert

code = '''
struct {pascal} {{ }}

fn pc_{snake}(code: &[u8]) -> Pr<{pascal}> {{
    todo!()
}}
'''

name = sys.argv[1]
snake = case_convert.snake_case(name)
pascal = case_convert.pascal_case(name)

print(code.format(snake=snake, pascal=pascal))
