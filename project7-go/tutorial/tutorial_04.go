package tutorial

import (
	"fmt"
	"golang.org/x/tour/reader"
	"io"
	"math"
	"strconv"
	"strings"
)

type VertexFloat struct {
	X, Y float64
}

func (v *VertexFloat) abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

func abs(v VertexFloat) float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

func (v *VertexFloat) scale(scaledBy float64) {
	v.X *= scaledBy
	v.Y *= scaledBy
}

func Scale(v *VertexFloat, f float64) {
	v.X = v.X * f
	v.Y = v.Y * f
}

type Abser interface {
	Abs() float64
}

func (v *VertexFloat) Abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

type I interface {
	M()
}

type T struct {
	S string
}

func (t *T) M() {
	if t == nil {
		fmt.Println("<nil>")
		return
	}
	fmt.Println(t.S)
}

type F float64

func (f F) M() {
	fmt.Println(f)
}

func describe(i I) {
	// %v -- value, %T -- type
	fmt.Printf("(%v, %T)\n", i, i)
}

func do(i interface{}) {
	switch v := i.(type) { // only allowed in switch as a whole type-switch expression
	case int:
		fmt.Printf("Twice %v is %v\n", v, v*2)
	case string:
		fmt.Printf("%q is %v bytes long\n", v, len(v))
	default:
		fmt.Printf("I don't know about type %T!\n", v)
	}
}

type Person struct {
	Name string
	Age  int
}

func (p Person) String() string {
	return fmt.Sprintf("%v (%v years)", p.Name, p.Age)
}

type IPAddr [4]byte

func (ip IPAddr) String() string {
	return fmt.Sprintf("%d.%d.%d.%d", ip[0], ip[1], ip[2], ip[3])
}

// ErrNegativeSqrt defining new error. It's an error because it implements Error() method
type ErrNegativeSqrt float64

func (e ErrNegativeSqrt) Error() string {
	return fmt.Sprintf("cannot Sqrt negative number: %f", e)
}

func Sqrt2(x float64) (float64, error) {

	if x < 0 {
		return 0, ErrNegativeSqrt(x)
	}
	z := x
	precision := 0.0001

	for (z*z)-x > precision {
		z -= (z*z - x) / (2 * z)
	}
	return z, nil
}

type MyReader struct{}

func (MyReader) Read(byteArray []byte) (int, error) {
	for i := range byteArray {
		byteArray[i] = 'A'
	}
	return len(byteArray), nil
}

type rot13Reader struct {
	r io.Reader
}

func (reader rot13Reader) Read(bytes []byte) (int, error) {
	// first read from existing reader, then update the output
	n, err := reader.r.Read(bytes)
	// forward by 13 positions
	for i, x := range bytes[:n] {
		// apply rot13 only to letters, else leave a byte as it is
		if x >= 'a' && x <= 'z' {
			bytes[i] = (x-'a'+13)%26 + 'a'
		} else if x >= 'A' && x <= 'Z' {
			bytes[i] = (x-'A'+13)%26 + 'A'
		}
	}

	return len(bytes), err
}

func CourseOfGoPartFour() {
	fmt.Println("Starting 4")

	vrtx := VertexFloat{1, 2}
	fmt.Println("vrtx", vrtx, "vrtx.abs()", vrtx.abs())
	fmt.Println("vrtx", vrtx, "abs(vrtx)", abs(vrtx))

	vrtx.scale(2)
	fmt.Println("vrtx after scale by 2", vrtx)

	Scale(&vrtx, 2)
	fmt.Println("vrtx after scale by 2 again", vrtx)

	(&vrtx).scale(2)
	fmt.Println("vrtx after scale by 2 yet again", vrtx)

	vrtx2 := &VertexFloat{3, 4}
	abser := Abser(vrtx2)
	fmt.Println("print vrtx2 as abser", abser)
	fmt.Println("****")

	// the t is nil, but we can still call the .M() method on it and not received an NPE
	// this is okay in Go
	var i I
	var t *T
	i = t
	describe(i)
	i.M()
	i = &T{"hello"}
	describe(i)
	i.M()

	var nilInterface I
	describe(nilInterface) // hold nil as both value and type

	// the next call will result in runtime error
	// nilInterface.M() calling a method on nil is not allowed -> runtime error. You need at least a type information available
	fmt.Println("***")

	// interface{} is an empty interface, it can hold any type. Every type implements at least empty interface
	var emtpyI interface{}
	fmt.Printf("(%v, %T)\n", emtpyI, emtpyI)

	var theInterface interface{} = "hello"

	s := theInterface.(string)
	fmt.Println(s)

	s, ok := theInterface.(string)
	fmt.Println(s, ok)

	// this would've panicked, but we have ok here, so it doesn't and simply return (0 false)
	f, ok := theInterface.(float64)
	fmt.Println(f, ok)

	// this will panic because theInterface is not float64
	// f = theInterface.(float64) // panic

	fmt.Println("****")
	do(21)
	do("hello")
	do(true)

	fmt.Println("****")
	person := Person{"Nosferatu", 132}
	fmt.Println(person)

	hosts := map[string]IPAddr{
		"loopback":  {127, 0, 0, 1},
		"googleDNS": {8, 8, 8, 8},
	}
	for name, ip := range hosts {
		fmt.Printf("%v: %v\n", name, ip)
	}

	fmt.Println("****")

	result, err := strconv.Atoi("42o")
	if err != nil {
		fmt.Printf("couldn't convert number: %v\n", err)
	}
	fmt.Println("Converted integer:", result)
	fmt.Println("****")

	fmt.Println(Sqrt2(2))
	fmt.Println(Sqrt2(-2))
	fmt.Println("****")

	r := strings.NewReader("Hello, Reader!")

	b := make([]byte, 8)
	for {
		n, err := r.Read(b)
		fmt.Printf("n = %v err = %v b = %v\n", n, err, b)
		fmt.Printf("b[:n] = %q\n", b[:n])
		if err == io.EOF {
			break
		}
	}
	reader.Validate(MyReader{})

	myReader := MyReader{}
	bytes := make([]byte, 24)
	readChars, err := myReader.Read(bytes)
	fmt.Printf("read bytes: %d, result: %s\n", readChars, bytes)
	if err == nil {
		fmt.Println("no error")
	} else {
		fmt.Println("error")
	}

	// ignore, because output too large
	// ss := strings.NewReader("Lbh penpxrq gur pbqr!")
	// rr := rot13Reader{ss}
	//io.Copy(os.Stdout, &rr)
}
