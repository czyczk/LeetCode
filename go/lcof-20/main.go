package main

import "fmt"

func main() {
	// Expecting true
	fmt.Println(isNumber("0"))
	// Expecting false
	fmt.Println(isNumber("e"))
	// Expecting false
	fmt.Println(isNumber("."))
	// Expecting true
	fmt.Println(isNumber(".1"))
	// Expecting true
	fmt.Println(isNumber("      .1  "))
	// Expecting true
	fmt.Println(isNumber("+100"))
	// Expecting true
	fmt.Println(isNumber("5e2"))
	// Expecting true
	fmt.Println(isNumber("-123"))
	// Expecting true
	fmt.Println(isNumber("3.1416"))
	// Expecting true
	fmt.Println(isNumber("-1E-16"))
	// Expecting true
	fmt.Println(isNumber("0123"))
	// Expecting false
	fmt.Println(isNumber("12e"))
	// Expecting false
	fmt.Println(isNumber("1a3.14"))
	// Expecting false
	fmt.Println(isNumber("1.2.3"))
	// Expecting false
	fmt.Println(isNumber("+-5"))
	// Expecting false
	fmt.Println(isNumber("12e+5.4"))
	// Expecting false
	fmt.Println(isNumber(" "))
	// Expecting false
	fmt.Println(isNumber("e9"))
	// Expecting true
	fmt.Println(isNumber("3."))
	// Expecting true
	fmt.Println(isNumber("-54.53061"))
	// Expecting true
	fmt.Println(isNumber(" -54.53061"))
}

func isNumber(s string) bool {
	n := len(s)
	isExpPart := false
	idxExpPart := -1
	isPointExisting := false
	isNumberBeforePointFilled := false
	isNumberAfterPointFilled := false
	isNumberAfterExpFilled := false
	isEnding := false

	i := 0
	for s[i] == ' ' {
		i++

		if i == n {
			// 空字符串
			return false
		}
	}

	idxFirstChar := i

	for ; i < n; i++ {
		ch := s[i]
		if ch == '+' || ch == '-' {
			if isEnding {
				return false
			}

			// 指数前和指数后均只有开头位置允许有符号
			allowedIdx := idxFirstChar
			if isExpPart {
				allowedIdx = idxExpPart + 1
			}

			if i != allowedIdx {
				return false
			}
		} else if ch == '.' {
			if isEnding {
				return false
			}

			if isExpPart || isPointExisting {
				// 指数部分不能有小数点。小数点不能出现多次。
				return false
			} else {
				// 标记已遇到小数点
				isPointExisting = true
			}
		} else if ch == 'e' || ch == 'E' {
			if isEnding {
				return false
			}

			if isExpPart {
				// 指数标志只能出现一次
				return false
			} else {
				if !isNumberBeforePointFilled && !isNumberAfterPointFilled {
					// 进入指数部分前小数部分必须合法
					return false
				}

				// 进入指数部分状态，记录状态与指数标志位置，并重置数值已填充标志
				isExpPart = true
				idxExpPart = i
			}
		} else if ch >= '0' && ch <= '9' {
			if isEnding {
				return false
			}

			if isExpPart {
				isNumberAfterExpFilled = true
			} else {
				if isPointExisting {
					isNumberAfterPointFilled = true
				} else {
					isNumberBeforePointFilled = true
				}
			}
		} else if ch == ' ' {
			// 只有开头和结尾能允许空格
			isEnding = true
		} else {
			// 不合法字符
			return false
		}
	}

	// 小数和指数部分必须合法
	if !isNumberBeforePointFilled && !isNumberAfterPointFilled {
		return false
	}
	if isExpPart && !isNumberAfterExpFilled {
		return false
	}

	return true
}
