size_t p4nenc8(       uint8_t  *in, size_t n, unsigned char *out);
size_t p4nenc16(      uint16_t *in, size_t n, unsigned char *out);
size_t p4nenc32(      uint32_t *in, size_t n, unsigned char *out);
size_t p4nenc64(      uint64_t *in, size_t n, unsigned char *out);
size_t p4nenc128v16(  uint16_t *in, size_t n, unsigned char *out);
size_t p4nenc128v32(  uint32_t *in, size_t n, unsigned char *out);
size_t p4nenc128v64(  uint64_t *in, size_t n, unsigned char *out);
size_t p4nenc256w32(  uint32_t *in, size_t n, unsigned char *out);
size_t p4nenc256v32(  uint32_t *in, size_t n, unsigned char *out);

size_t p4ndenc8(      uint8_t  *in, size_t n, unsigned char *out);
size_t p4ndenc16(     uint16_t *in, size_t n, unsigned char *out);
size_t p4ndenc32(     uint32_t *in, size_t n, unsigned char *out);
size_t p4ndenc128v16( uint16_t *in, size_t n, unsigned char *out);
size_t p4ndenc128v32( uint32_t *in, size_t n, unsigned char *out);
size_t p4ndenc256v32( uint32_t *in, size_t n, unsigned char *out);
size_t p4ndenc64(     uint64_t *in, size_t n, unsigned char *out);

size_t p4nd1enc8(     uint8_t  *in, size_t n, unsigned char *out);
size_t p4nd1enc16(    uint16_t *in, size_t n, unsigned char *out);
size_t p4nd1enc32(    uint32_t *in, size_t n, unsigned char *out);
size_t p4nd1enc128v16(uint16_t *in, size_t n, unsigned char *out);
size_t p4nd1enc128v32(uint32_t *in, size_t n, unsigned char *out);
size_t p4nd1enc256v32(uint32_t *in, size_t n, unsigned char *out);
size_t p4nd1enc64(    uint64_t *in, size_t n, unsigned char *out);

size_t p4nzenc8(      uint8_t  *in, size_t n, unsigned char *out);
size_t p4nzenc16(     uint16_t *in, size_t n, unsigned char *out);
size_t p4nzenc32(     uint32_t *in, size_t n, unsigned char *out);
size_t p4nzenc128v16( uint16_t *in, size_t n, unsigned char *out);
size_t p4nzenc128v32( uint32_t *in, size_t n, unsigned char *out);
size_t p4nzenc256v32( uint32_t *in, size_t n, unsigned char *out);
size_t p4nzenc64(     uint64_t *in, size_t n, unsigned char *out);

size_t p4ndec8(       unsigned char *in, size_t n, uint8_t  *out);
size_t p4ndec16(      unsigned char *in, size_t n, uint16_t *out);
size_t p4ndec32(      unsigned char *in, size_t n, uint32_t *out);
size_t p4ndec64(      unsigned char *in, size_t n, uint64_t *out);
size_t p4ndec128v16(  unsigned char *in, size_t n, uint16_t *out);
size_t p4ndec128v32(  unsigned char *in, size_t n, uint32_t *out);
size_t p4ndec128v64(  unsigned char *in, size_t n, uint64_t *out);
size_t p4ndec256v32(  unsigned char *in, size_t n, uint32_t *out);

size_t p4nddec8(      unsigned char *in, size_t n, uint8_t  *out);
size_t p4nddec16(     unsigned char *in, size_t n, uint16_t *out);
size_t p4nddec32(     unsigned char *in, size_t n, uint32_t *out);
size_t p4nddec128v16( unsigned char *in, size_t n, uint16_t *out);
size_t p4nddec128v32( unsigned char *in, size_t n, uint32_t *out);
size_t p4nddec256w32( unsigned char *in, size_t n, uint32_t *out);
size_t p4nddec256v32( unsigned char *in, size_t n, uint32_t *out);
size_t p4nddec64(     unsigned char *in, size_t n, uint64_t *out);

size_t p4nd1dec8(     unsigned char *in, size_t n, uint8_t  *out);
size_t p4nd1dec16(    unsigned char *in, size_t n, uint16_t *out);
size_t p4nd1dec32(    unsigned char *in, size_t n, uint32_t *out);
size_t p4nd1dec128v16(unsigned char *in, size_t n, uint16_t *out);
size_t p4nd1dec128v32(unsigned char *in, size_t n, uint32_t *out);
size_t p4nd1dec256v32(unsigned char *in, size_t n, uint32_t *out);
size_t p4nd1dec64(    unsigned char *in, size_t n, uint64_t *out);

size_t p4nzdec8(      unsigned char *in, size_t n, uint8_t  *out);
size_t p4nzdec16(     unsigned char *in, size_t n, uint16_t *out);
size_t p4nzdec32(     unsigned char *in, size_t n, uint32_t *out);
size_t p4nzdec128v16( unsigned char *in, size_t n, uint16_t *out);
size_t p4nzdec128v32( unsigned char *in, size_t n, uint32_t *out);
size_t p4nzdec256v32( unsigned char *in, size_t n, uint32_t *out);
size_t p4nzdec64(     unsigned char *in, size_t n, uint64_t *out);