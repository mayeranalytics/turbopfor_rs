// vsencNN: compress array with n unsigned (NN bits in[n]) values to the buffer out. Return value = end of compressed output buffer out
unsigned char *vsenc8( unsigned char  *in, size_t n, unsigned char  *out);
unsigned char *vsenc16(unsigned short *in, size_t n, unsigned char  *out);
unsigned char *vsenc32(unsigned int   *in, size_t n, unsigned char  *out);
unsigned char *vsenc64(uint64_t       *in, size_t n, unsigned char  *out);

// vsdecNN: decompress buffer into an array of n unsigned values. Return value = end of compressed input buffer in
unsigned char *vsdec8( unsigned char  *in, size_t n, unsigned char  *out);
unsigned char *vsdec16(unsigned char  *in, size_t n, unsigned short *out);
unsigned char *vsdec32(unsigned char  *in, size_t n, unsigned int   *out);
unsigned char *vsdec64(unsigned char  *in, size_t n, uint64_t       *out);