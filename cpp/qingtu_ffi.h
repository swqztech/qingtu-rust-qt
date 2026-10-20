#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void qingtu_free_string(char* s);

/** format: 0 保持 / 1 JPEG / 2 PNG */
char* qingtu_compress_files(const char** paths, int count, int quality, int max_side,
                            int format, int custom_out, const char* out_dir);

char* qingtu_stitch_files(const char** paths, int count, int custom_out, const char* out_dir);

#ifdef __cplusplus
}
#endif
