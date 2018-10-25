//
// Created by aloxaf on 18-10-25.
//
// raw: https://github.com/UltimateHackingKeyboard/current-window-linux/blob/master/get-current-window.c

#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "common.h"

#define MAXSTR 1000

Display *display;
unsigned long window;
unsigned char *prop;

void check_status(int status, unsigned long window)
{
    if (status == BadWindow) {
        printf("window id # 0x%lx does not exists!", window);
        exit(1);
    }

    if (status != Success) {
        printf("XGetWindowProperty failed!");
        exit(2);
    }
}

unsigned char* get_string_property(char* property_name)
{
    Atom actual_type, filter_atom;
    int actual_format, status;
    unsigned long nitems, bytes_after;

    filter_atom = XInternAtom(display, property_name, True);
    status = XGetWindowProperty(display, window, filter_atom, 0, MAXSTR, False, AnyPropertyType,
                                &actual_type, &actual_format, &nitems, &bytes_after, &prop);
    check_status(status, window);
    return prop;
}

unsigned long get_long_property(char* property_name)
{
    get_string_property(property_name);
    unsigned long long_property = prop[0] + (prop[1]<<8) + (prop[2]<<16) + (prop[3]<<24);
    return long_property;
}

DLL_PUBLIC WindowInfo get_active_window()
{
    display = XOpenDisplay(NULL);

    int screen = XDefaultScreen(display);
    window = RootWindow(display, screen);
    window = get_long_property("_NET_ACTIVE_WINDOW");

    WindowInfo info;
    if (window == 0) {
        info.PID = 0;
        info.wm_class = NULL;
        info.wm_name = NULL;
    } else {
        info.PID = get_long_property("_NET_WM_PID");
        info.wm_class = (char *) get_string_property("WM_CLASS");
        info.wm_name = (char *) get_string_property("_NET_WM_NAME");
    }
    XCloseDisplay(display);
    return info;
}
