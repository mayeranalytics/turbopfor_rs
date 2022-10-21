# vsimple.h

header = """
// vsencNN: compress array with n unsigned (NN bits in[n]) values to the buffer out. Return value = end of compressed output buffer out
unsigned char *vsenc8( unsigned char  *__restrict in, size_t n, unsigned char  *__restrict out);
unsigned char *vsenc16(unsigned short *__restrict in, size_t n, unsigned char  *__restrict out);
unsigned char *vsenc32(unsigned       *__restrict in, size_t n, unsigned char  *__restrict out);
unsigned char *vsenc64(uint64_t       *__restrict in, size_t n, unsigned char  *__restrict out);

// vsdecNN: decompress buffer into an array of n unsigned values. Return value = end of compressed input buffer in
unsigned char *vsdec8( unsigned char  *__restrict in, size_t n, unsigned char  *__restrict out);
unsigned char *vsdec16(unsigned char  *__restrict in, size_t n, unsigned short *__restrict out);
unsigned char *vsdec32(unsigned char  *__restrict in, size_t n, unsigned       *__restrict out);
unsigned char *vsdec64(unsigned char  *__restrict in, size_t n, uint64_t       *__restrict out);
"""