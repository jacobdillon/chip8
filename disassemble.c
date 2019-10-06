#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <endian.h>

char* c8tostr(uint16_t i)
{
	char *str = calloc(80, sizeof(char));

	switch ((i & 0xF000) >> 12) {
	case 0x0:
		switch (i & 0xFFFF) {
		case 0x00E0:
			sprintf(str, "CLS");
			break;
		case 0x00EE:
			sprintf(str, "RET");
		}
		break;
	case 0x1:
		sprintf(str, "J %03X", i & 0x0FFF);
		break;
	case 0x2:
		sprintf(str, "EXEC %03X", i & 0x0FFF);
		break;
	case 0x3:
		sprintf(str, "SEV $%1X, %02X", (i & 0x0F00) >> 8, i & 0x00FF);
		break;
	case 0x4:
		sprintf(str, "SNEV $%1X, %02X", (i & 0x0F00) >> 8, i & 0x00FF);
		break;
	case 0x5:
		sprintf(str, "SE $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
		break;
	case 0x6:
		sprintf(str, "SETV $%1X, %02X", (i & 0x0F00) >> 8, i & 0x00FF);
		break;
	case 0x7:
		sprintf(str, "ADDV $%1X, %02X", (i & 0x0F00) >> 8, i & 0x00FF);
		break;
	case 0x8:
		switch (i & 0x000F) {
		case 0x0:
			sprintf(str, "SET $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
			break;
		case 0x1:
			sprintf(str, "OR $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
			break;
		case 0x2:
			sprintf(str, "AND $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
			break;
		case 0x3:
			sprintf(str, "XOR $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
			break;
		case 0x4:
			sprintf(str, "ADD $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
			break;
		case 0x5:
			sprintf(str, "SUBL $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
			break;
		case 0x6:
			sprintf(str, "SHR $%1X", (i & 0x0F00) >> 8);
			break;
		case 0x7:
		        sprintf(str, "SUBR $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
			break;
		case 0xE:
			sprintf(str, "SHL $%1X", (i & 0x0F00) >> 8);
			break;								
		}
	case 0x9:
		sprintf(str, "SNE $%1X, $%1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4);
		break;
	case 0xA:
		sprintf(str, "LIV %03X", (i & 0x0FFF));
		break;
	case 0xC:
		sprintf(str, "RAND $%1X, %02X", (i & 0x0F00) >> 8, i & 0x00FF);
		break;
	case 0xD:
		sprintf(str, "DRAW $%1X, $%1X, %1X", (i & 0x0F00) >> 8, (i & 0x00F0) >> 4, i & 0x000F);
		break;
	case 0xE:
		switch (i & 0x00FF) {
		case 0x9E:
			sprintf(str, "SKP $%1X", (i & 0x0F00) >> 8);
			break;
		case 0xA1:
			sprintf(str, "SNKP $%1X", (i & 0x0F00) >> 8);
			break;
		}
	case 0xF:
		switch (i & 0x00FF) {
		case 0x07:
			sprintf(str, "LDT $%1X", (i & 0x0F00) >> 8);
			break;
		case 0x0A:
			sprintf(str, "WKP $%1X", (i & 0x0F00) >> 8);	
			break;
		case 0x15:
			sprintf(str, "SDT $%1X", (i & 0x0F00) >> 8);
			break;
		case 0x18:
			sprintf(str, "SST $%1X", (i & 0x0F00) >> 8);
			break;
		case 0x29:
			sprintf(str, "LI $%1X", (i & 0x0F00) >> 8);
			break;
		case 0x33:
			sprintf(str, "SBCD $%1X", (i & 0x0F00) >> 8);
			break;
		case 0x55:
			sprintf(str, "SRI $%1X", (i & 0x0F00) >> 8);
			break;
		case 0x65:
			sprintf(str, "LRI $%1X", (i & 0x0F00) >> 8);
			break;
		}
	}
	
	return str;
}

int main(int argc, char **argv)
{
	if (argc != 2) {
		fprintf(stderr, "usage: disassemble input_file\n");
		return 1;
	}

	FILE *fp = fopen(argv[1], "r");
	if (!fp) {
		fprintf(stderr, "failed to open file %s\n", argv[1]);
		return 2;
	}

	uint16_t i;
	do {
		fread(&i, 2, 1, fp);
		i = be16toh(i);
		char *istr = c8tostr(i);
		printf("%04X: %s\n", i, istr);
		free(istr);
	} while (i);
}
