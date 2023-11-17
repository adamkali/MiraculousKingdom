package src

import "math/rand"

func CreateUniqueID() string {
	str := "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	bytes := []byte(str)
	result := make([]byte, 16)
	for i := 0; i < 16; i++ {
		result[i] = bytes[rand.Intn(len(bytes))]
	}
	return string(result)
}
