#include <stdio.h>
#include <stdlib.h>

void printZ(long long a) {
    printf("%lld\n", a);
}

void printR(long double a) {
    printf("%Lf\n", a);
}

void printText(char* text) {
    printf("%s\n", text);
}

long long readZ() {
    long long l = 0;
    scanf("%lld", &l);
    return l;
}

long double readR() {
    long double l = 0;
    scanf("%Lf", &l);
    return l;
}

char* readText() {
    char* text = malloc(256);
    scanf("%s", text);
    return text;
}
