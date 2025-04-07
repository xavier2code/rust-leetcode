/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
///
///
///
#[allow(unused)]
pub fn roman_to_integer(s: String) -> i32 {
    // 创建一个哈希表，存储罗马数字和对应的整数值
    let mut roman_map = std::collections::HashMap::new();
    roman_map.insert(' ', 0);
    roman_map.insert('I', 1);
    roman_map.insert('V', 5);
    roman_map.insert('X', 10);
    roman_map.insert('L', 50);
    roman_map.insert('C', 100);
    roman_map.insert('D', 500);
    roman_map.insert('M', 1000);
    // 将字符串转换为字符数组
    let chars: Vec<char> = s.chars().collect();
    // 初始化结果变量
    let mut result = 0;
    // 遍历字符数组
    for i in 0..chars.len() {
        // 获取当前字符和下一个字符
        let current = chars[i];
        let next = if i + 1 < chars.len() {
            chars[i + 1]
        } else {
            ' '
        };
        // 如果当前字符的值小于下一个字符的值，则减去当前字符的值
        if roman_map[&current] < roman_map[&next] {
            result -= roman_map[&current];
        } else {
            // 否则加上当前字符的值
            result += roman_map[&current];
        }
    }
    // 返回结果
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer() {
        assert_eq!(roman_to_integer("III".to_string()), 3);
        assert_eq!(roman_to_integer("IV".to_string()), 4);
        assert_eq!(roman_to_integer("IX".to_string()), 9);
        assert_eq!(roman_to_integer("LVIII".to_string()), 58);
        assert_eq!(roman_to_integer("MCMXCIV".to_string()), 1994);
    }
}
