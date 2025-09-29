// 字符串测试
package main

import "fmt"

func TestString() string {
	a := "Hello, Go!"
	b := a
	c := a + " +++++"
	fmt.Println(b, c)
	return b + " " + c
}