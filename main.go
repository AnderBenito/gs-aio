package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run hexviewer.go <gba-file>")
		return
	}

	// Open the GBA file
	fileName := os.Args[1]
	file, err := os.Open(fileName)
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		return
	}
	defer file.Close()

	reader := bufio.NewReader(file)
	const bytesPerLine = 16 // Number of bytes per line in the hex view
	// offset := 0

	rom, err := io.ReadAll(reader)
	if err != nil {
		panic(err)
	}

	name := getString(rom, 0xA0, 15)
	language := string(rom[0xAF])
	fmt.Println(name, language)

	// fmt.Println(rom[offset:bytesPerLine])

	// // Print hex representation
	// hexData := hex.EncodeToString(rom[:bytesPerLine])
	// for i := 0; i < len(hexData); i += 2 {
	// 	fmt.Printf("%s ", hexData[i:i+2])
	// }

	// ct := make([]int32, 0x100)
	// for i := range ct {
	// 	ct[i] = -1 // Initialize all to -1 (default Golang int value is 0)
	// }

	// // Populate ct array
	// for i := int32(0); i < 3159-2915; i++ {
	// 	offset := 0xC15F4 + i*0x54
	// 	n := getInt32(rom, offset)
	// 	if n < 0 || n >= 0x100 {
	// 		continue
	// 	}
	// 	if ct[n] == -1 {
	// 		ct[n] = 2915 + i
	// 	}
	// }

	// // ctstr array initialization
	// ctstr := make([]string, 0x100)

	// txt := make([]byte, 0xFFFFF) // Mocked text data

	// // Populate ctstr array
	// for i := 0; i < 0x100; i++ {
	// 	if ct[i] != -1 {
	// 		ctstr[i] = fmt.Sprintf("%3d|%s", i, getTextShort(txt, ct[i]))
	// 	} else {
	// 		ctstr[i] = fmt.Sprintf("%3d|", i)
	// 	}
	// }

	// // Print results (for demonstration purposes)
	// for i, s := range ctstr {
	// 	fmt.Println(i, s)
	// }

	text := decompText(rom)

	showClasses(rom, text)

	os.WriteFile("output.txt", text, 0644)

	// for {
	// 	// Read a chunk of the file
	// 	buffer := make([]byte, bytesPerLine)
	// 	n, err := reader.Read(buffer)
	// 	if n == 0 {
	// 		break // End of file
	// 	}

	// 	// Print offset
	// 	fmt.Printf("%08X  ", offset)

	// 	// Print hex representation
	// 	hexData := hex.EncodeToString(buffer[:n])
	// 	for i := 0; i < len(hexData); i += 2 {
	// 		fmt.Printf("%s ", hexData[i:i+2])
	// 	}

	// 	// Pad the line if it's shorter than bytesPerLine
	// 	if n < bytesPerLine {
	// 		fmt.Print(strings.Repeat("   ", (bytesPerLine - n)))
	// 	}

	// 	// Print ASCII representation
	// 	fmt.Print(" |")
	// 	for i := 0; i < n; i++ {
	// 		if buffer[i] >= 32 && buffer[i] <= 126 {
	// 			fmt.Printf("%c", buffer[i]) // Printable ASCII characters
	// 		} else {
	// 			fmt.Print(".") // Non-printable characters
	// 		}
	// 	}
	// 	fmt.Println("|")

	// 	offset += n

	// 	// Break on EOF
	// 	if err != nil {
	// 		break
	// 	}
	// }
}

func getInt32(buffer []byte, pos int32) int32 {
	return int32(buffer[pos]) |
		int32(buffer[pos+1])<<8 |
		int32(buffer[pos+2])<<16 |
		int32(buffer[pos+3])<<24
}

func getString(src []byte, pos int, length int) string {
	var strbuild []rune

	for length > 0 {
		strbuild = append(strbuild, rune(src[pos]))
		pos++
		length--
	}

	return string(strbuild)
}

func getTextShort(txt []byte, index int32) string {
	var p, n byte
	var str strings.Builder

	if index < 0 {
		return "" // Return an empty string for negative indices
	}

	// Get source position
	srcPos := int(getInt32(txt, index<<2))
	if getInt32(txt, 0) == 0 {
		srcPos += 0xC300
	}

	// Read text data
	for {
		n = txt[srcPos]
		srcPos++

		if n != 0 {
			if n < 32 || n > 0x7E { // Non-printable or special characters
				str.WriteString(fmt.Sprintf("[%d]", n))

				// Commands with arguments: skip certain conditions
				if (n == 1 || n == 3) && (p < 17 || p > 20) && p != 26 && p != 29 {
					n = 0
				}
			} else {
				str.WriteByte(n) // Append printable characters
			}
		}

		p = n
		if n == 0 {
			break // End of string
		}
	}

	return str.String()
}

func getInt16(buffer []byte, pos int32) int32 {
	pos1 := int(buffer[pos])
	pos2 := int(buffer[pos+1])
	fpos := pos1 | pos2<<8
	return int32(fpos)
}

func setInt32(buffer []byte, pos int32, value int32) {
	buffer[pos] = byte(value)
	buffer[pos+1] = byte(value >> 8)
	buffer[pos+2] = byte(value >> 16)
	buffer[pos+3] = byte(value >> 24)
}

func showClasses(src []byte, txt []byte) {
	// items := make([]int32, 0)
	// for i := int32(0); i < 3159-2915; i++ {
	// 	items = append(items, (2915+461)+i)
	// }

	// for _, val := range items {
	// 	className := getTextShort(txt, val)
	// 	fmt.Println(className)
	// }

	// // Get psy names
	// psyNames := make([]string, 734)
	// for i := int32(0); i < 734; i++ {
	// 	psyName := getTextShort(txt, i+(1447+461))
	// 	psyNames = append(psyNames, psyName)
	// 	fmt.Println(psyName)
	// }

	// // ENEMY ENCOUNTERS
	// // address := 0xEDACC
	// // THIS FOR SPANISH VERSION
	// address := int32(0xF6AC0)
	// // THIS FOR SPANISH VRSION
	// pointerAddress := int32(0x12CE94)
	// items := make([]int32, 0)
	// for i := int32(0); i < 0x6E; i++ {
	// 	ind2 := getInt16(src, address+4+i*0x1C)
	// 	ind := getInt16(src, (pointerAddress)+ind2*0x18)

	// 	items = append(items, (1068+461)+ind)
	// }

	// for _, val := range items {
	// 	className := getTextShort(txt, val)
	// 	fmt.Println(className)
	// }

	// Ability
	// Spain
	abiltyTableAddress := int32(0xC0C20)
	// abilityItems := make([]int32, 0)

	for i := int32(0); i < 734; i++ {
		// 12 bytes per entry
		power := getInt16(src, (abiltyTableAddress)+10+i*0xC)
		fmt.Println(power)
	}
}

func decompText(src []byte) []byte {
	total := int32(0)
	asmpchar := getInt32(src, 0x38578) - 0x8000000
	asmptext := getInt32(src, 0x385DC) - 0x8000000
	chardata := getInt32(src, asmpchar) - 0x08000000
	charpntrs := getInt32(src, asmpchar+4) - 0x08000000

	maxLetter := int32(0)
	cTreeSize := int32(0)
	maxDepth := int32(0)

	// Pre-scan character tables
	for char1 := int32(0); char1 <= maxLetter; char1++ {
		if char1&0xFF == 0 {
			chardata = getInt32(src, asmpchar+(char1>>8)*8) - 0x08000000
			charpntrs = getInt32(src, asmpchar+(char1>>8)*8+4) - 0x08000000
		}
		cmp := int32(getInt16(src, charpntrs))
		if cmp == 0x8000 {
			charpntrs += 2
			continue
		}
		charTree := (chardata + int32(getInt16(src, charpntrs))) << 3
		charpntrs += 2
		charSlot := charTree - 12
		depth := int32(0)

		for {
			for {
				if ((src[charTree>>3] >> (charTree & 7)) & 1) != 0 {
					charTree++
					break
				}
				depth++
				cTreeSize++
				// Increment happens after evaluating the condition
				charTree++
			}
			charSlotPos := charSlot >> 3
			if total >= 265283 {
				fmt.Println("a")
			}
			letter := (getInt16(src, charSlotPos) >> (charSlot & 7)) & 0xFFF
			charSlot -= 12
			total++
			cTreeSize++
			// TODO
			if int32(letter) > maxLetter {
				maxLetter = int32(letter)
			}
			if depth > maxDepth {
				maxDepth = depth
			}
			if depth <= 0 {
				break
			}
			depth--
		}
	}
	fmt.Println("TOTAL", total)

	// Initialize arrays
	ctOffsets := make([]int32, maxLetter+1)
	cTree := make([]int32, cTreeSize)
	nodeOffsets := make([]int32, maxDepth)

	pos := int32(0)
	for char1 := int32(0); char1 <= maxLetter; char1++ {
		if char1&0xFF == 0 {
			chardata = getInt32(src, asmpchar+(char1>>8)*8) - 0x08000000
			charpntrs = getInt32(src, asmpchar+(char1>>8)*8+4) - 0x08000000
		}
		// TODO
		if int(getInt16(src, charpntrs)) == 0x8000 {
			charpntrs += 2
			continue
		}
		charTree := (chardata + int32(getInt16(src, charpntrs))) << 3
		charpntrs += 2
		charSlot := charTree - 12
		depth := 0

		ctOffsets[char1] = pos
		for {
			for (src[charTree>>3]>>(charTree&7))&1 == 0 {
				nodeOffsets[depth] = pos
				depth++
				pos++
				charTree++
			}
			charTree++
			// TODO
			cTree[pos] = -((getInt16(src, charSlot>>3) >> (charSlot & 7)) & 0xFFF)
			pos++
			charSlot -= 12

			if depth <= 0 {
				break
			}
			depth--
			cTree[nodeOffsets[depth]] = pos
		}
	}

	// Decompression
	des := make([]byte, 0x800000)
	desEntry := int32(0)
	desPos := 0xC300

	textTree := int32(0)
	textLenAddr := int32(0)
	for srcI := int32(0); srcI < 12461; srcI++ {
		setInt32(des, desEntry, int32(desPos-0xC300))
		desEntry += 4

		srcInd := srcI
		if srcInd&0xFF == 0 {
			textTree = getInt32(src, asmptext+(srcInd>>8)<<3) - 0x08000000
			textLenAddr = getInt32(src, asmptext+((srcInd>>8)<<3)+4) - 0x08000000
		} else {
			cLen := int32(0)
			for {
				cLen = int32(src[textLenAddr])
				textLenAddr++
				textTree += cLen
				if cLen != 0xFF {
					break
				}
			}
		}
		initChar := int32(0)
		textTree2 := textTree << 3

		for {
			pos = ctOffsets[initChar]
			for cTree[pos] > 0 {
				if (src[textTree2>>3]>>(textTree2&7))&1 == 0 {
					pos++
				} else {
					pos = cTree[pos]
				}
				textTree2++
			}
			initChar = -cTree[pos]
			des[desPos] = byte(initChar)
			desPos++
			if initChar == 0 {
				break
			}
		}
	}

	return des
}
