//
// Created by aloxaf on 18-10-25.
//

#ifndef RUST_TOYS_COMMON_H
#define RUST_TOYS_COMMON_H

#if defined _WIN32 || defined __CYGWIN__
#ifdef BUILDING_DLL
    #ifdef __GNUC__
      #define DLL_PUBLIC __attribute__ ((dllexport))
    #else
      #define DLL_PUBLIC __declspec(dllexport) // Note: actually gcc seems to also supports this syntax.
    #endif
  #else
    #ifdef __GNUC__
      #define DLL_PUBLIC __attribute__ ((dllimport))
    #else
      #define DLL_PUBLIC __declspec(dllimport) // Note: actually gcc seems to also supports this syntax.
    #endif
  #endif
  #define DLL_LOCAL
#else
#if __GNUC__ >= 4
#define DLL_PUBLIC __attribute__ ((visibility ("default")))
    #define DLL_LOCAL  __attribute__ ((visibility ("hidden")))
#else
#define DLL_PUBLIC
#define DLL_LOCAL
#endif
#endif

typedef struct {
    unsigned long PID;
    char *wm_class;
    char *wm_name;
} WindowInfo;

typedef struct {
    int type;
    unsigned int keycode;
} KeyEvent;

#endif //RUST_TOYS_COMMON_H