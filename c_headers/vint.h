
// ---- Variable byte length after compression
unsigned int vblen16(unsigned short x);
unsigned int vblen32(unsigned int   x);
unsigned int vblen64(uint64_t       x);

// ---- Length of compressed value. Input in is the first char of the compressed buffer start (Ex. vbvlen32(in[0]) )
unsigned int vbvlen16(unsigned int x);
unsigned int vbvlen32(unsigned int x);
unsigned int vbvlen64(unsigned int x);

//----------------------------- TurboVByte 'vb':Variable byte + SIMD TurboByte 'v8': array functions ----------------------------------------
// Encoding/DEcoding: Return value = end of compressed output/input buffer out/in

//----------------------- Encoding/Decoding unsorted array with n integer values --------------------------
unsigned char *vbenc16(  unsigned short *in, unsigned int n, unsigned char  *out);
unsigned char *vbenc32(  unsigned int   *in, unsigned int n, unsigned char  *out);
unsigned char *vbenc64(  uint64_t       *in, unsigned int n, unsigned char  *out);

//-- Decode
unsigned char *vbdec16(  unsigned char  *in, unsigned int n, unsigned short *out);
unsigned char *vbdec32(  unsigned char  *in, unsigned int n, unsigned int   *out);
unsigned char *vbdec64(  unsigned char  *in, unsigned int n, uint64_t       *out);

//-- Get value stored at index idx (idx:0...n-1)
unsigned short vbgetx16(  unsigned char *in, unsigned int idx);
unsigned int   vbgetx32(  unsigned char *in, unsigned int idx);
uint64_t       vbgetx64(  unsigned char *in, unsigned int idx);

//-- Search and return index of next value equal to key or n when no key value found
// ex. unsigned int idx;unsigned char *ip; for(idx=0,ip=in;;) { if((idx = vgeteq32(&ip, idx, 4321))>=n) break; printf("found at %u ", idx); }
unsigned int vbgeteq16( unsigned char **in, unsigned int n, unsigned int idx, unsigned short key);
unsigned int vbgeteq32( unsigned char **in, unsigned int n, unsigned int idx, unsigned int   key);
unsigned int vbgeteq64( unsigned char **in, unsigned int n, unsigned int idx, uint64_t       key);

//---------------------- Delta encoding/decoding sorted array ---------------------------------------------
//-- Increasing integer array. out[i] = out[i-1] + in[i]
unsigned char *vbdenc16( unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *vbdenc32( unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);
unsigned char *vbdenc64( uint64_t       *in, unsigned int n, unsigned char  *out, uint64_t       start);

unsigned char *vbddec16( unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *vbddec32( unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);
unsigned char *vbddec64( unsigned char  *in, unsigned int n, uint64_t       *out, uint64_t       start);

//-- Get value stored at index idx (idx:0...n-1)
unsigned short vbdgetx16(  unsigned char *in, unsigned int idx, unsigned short start);
unsigned int   vbdgetx32(  unsigned char *in, unsigned int idx, unsigned int start);
uint64_t       vbdgetx64(  unsigned char *in, unsigned int idx, uint64_t start);

//-- Search and return index of next value equal to key or n when no key value found
// ex. unsigned int idx;unsigned char *ip; for(idx=0,ip=in;;) { if((idx = vgeteq32(&ip, idx, 4321))>=n) break; printf("found at %u ", idx); }
unsigned int vbdgetgeq16( unsigned char **in, unsigned int n, unsigned int idx, unsigned short *key, unsigned short start);
unsigned int vbdgetgeq32( unsigned char **in, unsigned int n, unsigned int idx, unsigned int   *key, unsigned int   start);
unsigned int vbdgetgeq64( unsigned char **in, unsigned int n, unsigned int idx, uint64_t       *key, uint64_t       start);

//-- Strictly increasing (never remaining constant or decreasing) integer array. out[i] = out[i-1] + in[i] + 1
unsigned char *vbd1enc16(unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *vbd1enc32(unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);
unsigned char *vbd1enc64(uint64_t       *in, unsigned int n, unsigned char  *out, uint64_t       start);

unsigned char *vbd1dec16(unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *vbd1dec32(unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);
unsigned char *vbd1dec64(unsigned char  *in, unsigned int n, uint64_t       *out, uint64_t       start);


//-- Get value stored at index idx (idx:0...n-1)
unsigned short vbd1getx16(  unsigned char *in, unsigned int idx, unsigned short start);
unsigned int   vbd1getx32(  unsigned char *in, unsigned int idx, unsigned int   start);
uint64_t       vbd1getx64(  unsigned char *in, unsigned int idx, uint64_t       start);

//-- Search and return index of next value equal to key or n when no key value found
// ex. unsigned int idx;unsigned char *ip; for(idx=0,ip=in;;) { if((idx = vgeteq32(&ip, idx, 4321))>=n) break; printf("found at %u ", idx); }
unsigned int vbd1getgeq16( unsigned char **in, unsigned int n, unsigned int idx, unsigned short *key, unsigned short start);
unsigned int vbd1getgeq32( unsigned char **in, unsigned int n, unsigned int idx, unsigned int   *key, unsigned int   start);
unsigned int vbd1getgeq64( unsigned char **in, unsigned int n, unsigned int idx, uint64_t       *key, uint64_t       start);

//---------------------- Zigzag encoding/decoding for unsorted integer lists.
unsigned char *vbzenc8(  unsigned char  *in, unsigned int n, unsigned char  *out, unsigned char start);
unsigned char *vbzenc16( unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *vbzenc32( unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);
unsigned char *vbzenc64( uint64_t       *in, unsigned int n, unsigned char  *out, uint64_t       start);

unsigned char *vbzdec8(  unsigned char  *in, unsigned int n, unsigned char  *out, unsigned char  start);
unsigned char *vbzdec16( unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *vbzdec32( unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);
unsigned char *vbzdec64( unsigned char  *in, unsigned int n, uint64_t       *out, uint64_t       start);

//---------------------- XOR encoding/decoding for unsorted integer lists.
unsigned char *vbxenc8(  unsigned char  *in, unsigned int n, unsigned char  *out, unsigned char start);
unsigned char *vbxenc16( unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *vbxenc32( unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);
unsigned char *vbxenc64( uint64_t       *in, unsigned int n, unsigned char  *out, uint64_t       start);

unsigned char *vbxdec8(  unsigned char  *in, unsigned int n, unsigned char  *out, unsigned char  start);
unsigned char *vbxdec16( unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *vbxdec32( unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);
unsigned char *vbxdec64( unsigned char  *in, unsigned int n, uint64_t       *out, uint64_t       start);

//---------------------- Delta of delta encoding/decoding for unsorted integer lists.
unsigned char *vbddenc16( unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *vbddenc32( unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);
unsigned char *vbddenc64( uint64_t       *in, unsigned int n, unsigned char  *out, uint64_t       start);

unsigned char *vbdddec16( unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *vbdddec32( unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);
unsigned char *vbdddec64( unsigned char  *in, unsigned int n, uint64_t       *out, uint64_t       start);

//-- Get value stored at index idx (idx:0...n-1)
unsigned short vbzgetx16(  unsigned char *in, unsigned int idx, unsigned short start);
unsigned int   vbzgetx32(  unsigned char *in, unsigned int idx, unsigned int   start);
uint64_t       vbzgetx64(  unsigned char *in, unsigned int idx, uint64_t       start);

//-------------------------- TurboByte (SIMD Group varint) --------------------------------------------------------------
unsigned char *v8enc16(  unsigned short *in, unsigned int n, unsigned char  *out); //TurboByte
unsigned char *v8enc32(  unsigned int   *in, unsigned int n, unsigned char  *out);

unsigned char *v8dec16(  unsigned char  *in, unsigned int n, unsigned short *out);
unsigned char *v8dec32(  unsigned char  *in, unsigned int n, unsigned int   *out);

//------ delta ---------
unsigned char *v8denc16( unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *v8denc32( unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);

unsigned char *v8ddec16( unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *v8ddec32( unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);

//------ delta 1 -------
unsigned char *v8d1enc16(unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *v8d1enc32(unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);

unsigned char *v8d1dec16(unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *v8d1dec32(unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);

//------- zigzag -------
unsigned char *v8zenc16( unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *v8zenc32( unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);

unsigned char *v8zdec16( unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *v8zdec32( unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);

//------- xor ----------
unsigned char *v8xenc16( unsigned short *in, unsigned int n, unsigned char  *out, unsigned short start);
unsigned char *v8xenc32( unsigned int   *in, unsigned int n, unsigned char  *out, unsigned int   start);

unsigned char *v8xdec16( unsigned char  *in, unsigned int n, unsigned short *out, unsigned short start);
unsigned char *v8xdec32( unsigned char  *in, unsigned int n, unsigned int   *out, unsigned int   start);
//-------------------------- TurboByte Hybrid (SIMD Group varint) + Bitpacking -------------------------------------------
size_t v8nenc16(  uint16_t *in, size_t n, unsigned char *out);
size_t v8nenc32(  uint32_t *in, size_t n, unsigned char *out);

size_t v8ndenc16( uint16_t *in, size_t n, unsigned char *out);
size_t v8ndenc32( uint32_t *in, size_t n, unsigned char *out);

size_t v8nd1enc16(uint16_t *in, size_t n, unsigned char *out);
size_t v8nd1enc32(uint32_t *in, size_t n, unsigned char *out);

size_t v8nzenc16( uint16_t *in, size_t n, unsigned char *out);
size_t v8nzenc32( uint32_t *in, size_t n, unsigned char *out);

size_t v8ndec16(  unsigned char *in, size_t n, uint16_t *out);
size_t v8ndec32(  unsigned char *in, size_t n, uint32_t *out);

size_t v8nddec16( unsigned char *in, size_t n, uint16_t *out);
size_t v8nddec32( unsigned char *in, size_t n, uint32_t *out);

size_t v8nd1dec16(unsigned char *in, size_t n, uint16_t *out);
size_t v8nd1dec32(unsigned char *in, size_t n, uint32_t *out);

size_t v8nzdec16( unsigned char *in, size_t n, uint16_t *out);
size_t v8nzdec32( unsigned char *in, size_t n, uint32_t *out);

size_t v8nxdec16( unsigned char *in, size_t n, uint16_t *out);
size_t v8nxdec32( unsigned char *in, size_t n, uint32_t *out);
//-------------
size_t v8nenc128v16(  uint16_t *in, size_t n, unsigned char *out);
size_t v8nenc128v32(  uint32_t *in, size_t n, unsigned char *out);

size_t v8ndenc128v16( uint16_t *in, size_t n, unsigned char *out);
size_t v8ndenc128v32( uint32_t *in, size_t n, unsigned char *out);

size_t v8nd1enc128v16(uint16_t *in, size_t n, unsigned char *out);
size_t v8nd1enc128v32(uint32_t *in, size_t n, unsigned char *out);

size_t v8nzenc128v16( uint16_t *in, size_t n, unsigned char *out);
size_t v8nzenc128v32( uint32_t *in, size_t n, unsigned char *out);

size_t v8ndec128v16(  unsigned char *in, size_t n, uint16_t *out);
size_t v8ndec128v32(  unsigned char *in, size_t n, uint32_t *out);

size_t v8nddec128v16( unsigned char *in, size_t n, uint16_t *out);
size_t v8nddec128v32( unsigned char *in, size_t n, uint32_t *out);

size_t v8nd1dec128v16(unsigned char *in, size_t n, uint16_t *out);
size_t v8nd1dec128v32(unsigned char *in, size_t n, uint32_t *out);

size_t v8nzdec128v16( unsigned char *in, size_t n, uint16_t *out);
size_t v8nzdec128v32( unsigned char *in, size_t n, uint32_t *out);

size_t v8nxdec128v16( unsigned char *in, size_t n, uint16_t *out);
size_t v8nxdec128v32( unsigned char *in, size_t n, uint32_t *out);
//-------------
size_t v8nenc256v32(  uint32_t *in, size_t n, unsigned char *out);
size_t v8ndenc256v32( uint32_t *in, size_t n, unsigned char *out);
size_t v8nd1enc256v32(uint32_t *in, size_t n, unsigned char *out);
size_t v8nzenc256v32( uint32_t *in, size_t n, unsigned char *out);
size_t v8nxenc256v32( uint32_t *in, size_t n, unsigned char *out);

size_t v8ndec256v32(  unsigned char *in, size_t n, uint32_t *out);
size_t v8nddec256v32( unsigned char *in, size_t n, uint32_t *out);
size_t v8nd1dec256v32(unsigned char *in, size_t n, uint32_t *out);
size_t v8nzdec256v32( unsigned char *in, size_t n, uint32_t *out);
size_t v8nxdec256v32( unsigned char *in, size_t n, uint32_t *out);