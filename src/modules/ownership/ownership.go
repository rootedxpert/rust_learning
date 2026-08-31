var count = 0;
go func() {
	count++
}
go func() {
	count++
}

// potential race condition