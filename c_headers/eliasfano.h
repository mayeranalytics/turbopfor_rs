unsigned char *efanoenc32(     unsigned int  *in, unsigned int n, unsigned char  *out, unsigned int start);
unsigned char *efanoenc64(     uint64_t      *in, unsigned int n, unsigned char  *out, uint64_t start);

unsigned char *efanodec32(     unsigned char *in, unsigned int n, unsigned int   *out, unsigned int start);
unsigned char *efanodec64(     unsigned char *in, unsigned int n, uint64_t       *out, uint64_t start);

unsigned char *efano1enc32(    unsigned int  *in, unsigned int n, unsigned char  *out, unsigned int start);
unsigned char *efano1enc64(    uint64_t      *in, unsigned int n, unsigned char  *out, uint64_t start);

unsigned char *efano1dec32(    unsigned char *in, unsigned int n, unsigned int   *out, unsigned int start);
unsigned char *efano1dec64(    unsigned char *in, unsigned int n, uint64_t       *out, uint64_t start);

unsigned char *efanoenc128v32( unsigned int  *in, unsigned int n, unsigned char  *out, unsigned int start);
unsigned char *efanodec128v32( unsigned char *in, unsigned int n, unsigned int   *out, unsigned int start);

unsigned char *efano1enc128v32(unsigned int  *in, unsigned int n, unsigned char  *out, unsigned int start);
unsigned char *efano1dec128v32(unsigned char *in, unsigned int n, unsigned int   *out, unsigned int start);

unsigned char *efanoenc256v32( unsigned int  *in, unsigned int n, unsigned char  *out, unsigned int start);
unsigned char *efanodec256v32( unsigned char *in, unsigned int n, unsigned int   *out, unsigned int start);

unsigned char *efano1enc256v32(unsigned int  *in, unsigned int n, unsigned char  *out, unsigned int start);
unsigned char *efano1dec256v32(unsigned char *in, unsigned int n, unsigned int   *out, unsigned int start);