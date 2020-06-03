
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void    seastar_app_template_run(void* ctx, int argc, char** argv);
void    seastar_spawn(void* ctx);
void    seastar_sleep(void* ctx, unsigned long ns);
void    seastar_file_size(void* ctx, char* fname);

void    seastar_rs_closure_void(void* ctx);
void    seastar_rs_closure_u32(void* ctx, int32_t);
void    seastar_rs_closure_u64(void* ctx, uint64_t);
void    seastar_rs_closure_voidptr(void* ctx, void*);

#ifdef __cplusplus
}
#endif
