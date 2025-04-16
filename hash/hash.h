#include <stdlib.h>  // for malloc, free
#include <string.h>  // for strlen, strcpy, strcmp

// Return codes
typedef enum tagReturnCode {
    SUCCESS,
    FAIL
} ReturnCode;

// Entry key-value pair
typedef struct tagEntry {
    char* key;
    char* value;
} Entry;

// Linked list node
typedef struct tagNode {
    Entry* entry;
    struct tagNode* next;
} Node;

// Hash table
typedef struct tagHash {
    unsigned int table_size;
    Node** heads;
} Hash;

// Function declarations
ReturnCode HashCreate(Hash** hash, unsigned int table_size);
ReturnCode HashInsert(Hash* hash, const Entry* entry);
const Entry* HashFind(const Hash* hash, const char* key);
ReturnCode HashRemove(Hash* hash, const char* key);
void HashPrint(Hash* hash, void (*PrintFunc)(char*, char*));
void HashDestroy(Hash* hash);
