# bitpack.h
header = """

size_t bitnpack8(         uint8_t  *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnpack16(        uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnpack32(        uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnpack64(        uint64_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnpack128v16(    uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnpack128v32(    uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnpack128v64(    uint64_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnpack256v32(    uint32_t *__restrict in, size_t n, unsigned char *__restrict out);

size_t bitndpack8(        uint8_t  *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitndpack16(       uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitndpack32(       uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitndpack64(       uint64_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitndpack128v16(   uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitndpack128v32(   uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitndpack256v32(   uint32_t *__restrict in, size_t n, unsigned char *__restrict out);

size_t bitnd1pack8(       uint8_t  *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnd1pack16(      uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnd1pack32(      uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnd1pack64(      uint64_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnd1pack128v16(  uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnd1pack128v32(  uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnd1pack256v32(  uint32_t *__restrict in, size_t n, unsigned char *__restrict out);

size_t bitnzpack8(        uint8_t  *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnzpack16(       uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnzpack32(       uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnzpack64(       uint64_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnzpack128v16(   uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnzpack128v32(   uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnzpack256v32(   uint32_t *__restrict in, size_t n, unsigned char *__restrict out);

size_t bitnfpack8(        uint8_t  *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnfpack16(       uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnfpack32(       uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnfpack64(       uint64_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnfpack128v16(   uint16_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnfpack128v32(   uint32_t *__restrict in, size_t n, unsigned char *__restrict out);
size_t bitnfpack256v32(   uint32_t *__restrict in, size_t n, unsigned char *__restrict out);

size_t bitnunpack8(       unsigned char *__restrict in, size_t n, uint8_t  *__restrict out);
size_t bitnunpack16(      unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnunpack32(      unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnunpack64(      unsigned char *__restrict in, size_t n, uint64_t *__restrict out);
size_t bitnunpack128v16(  unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnunpack128v32(  unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnunpack128v64(  unsigned char *__restrict in, size_t n, uint64_t *__restrict out);
size_t bitnunpack256v32(  unsigned char *__restrict in, size_t n, uint32_t *__restrict out);

size_t bitndunpack8(      unsigned char *__restrict in, size_t n, uint8_t  *__restrict out);
size_t bitndunpack16(     unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitndunpack32(     unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitndunpack64(     unsigned char *__restrict in, size_t n, uint64_t *__restrict out);
size_t bitndunpack128v16( unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitndunpack128v32( unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitndunpack256v32( unsigned char *__restrict in, size_t n, uint32_t *__restrict out);

size_t bitnd1unpack8(     unsigned char *__restrict in, size_t n, uint8_t  *__restrict out);
size_t bitnd1unpack16(    unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnd1unpack32(    unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnd1unpack64(    unsigned char *__restrict in, size_t n, uint64_t *__restrict out);
size_t bitnd1unpack128v16(unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnd1unpack128v32(unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnd1unpack256v32(unsigned char *__restrict in, size_t n, uint32_t *__restrict out);

size_t bitnzunpack8(      unsigned char *__restrict in, size_t n, uint8_t  *__restrict out);
size_t bitnzunpack16(     unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnzunpack32(     unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnzunpack64(     unsigned char *__restrict in, size_t n, uint64_t *__restrict out);
size_t bitnzunpack128v16( unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnzunpack128v32( unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnzunpack256v32( unsigned char *__restrict in, size_t n, uint32_t *__restrict out);

size_t bitnfunpack8(      unsigned char *__restrict in, size_t n, uint8_t  *__restrict out);
size_t bitnfunpack16(     unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnfunpack32(     unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnfunpack64(     unsigned char *__restrict in, size_t n, uint64_t *__restrict out);
size_t bitnfunpack128v16( unsigned char *__restrict in, size_t n, uint16_t *__restrict out);
size_t bitnfunpack128v32( unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
size_t bitnfunpack256v32( unsigned char *__restrict in, size_t n, uint32_t *__restrict out);
"""