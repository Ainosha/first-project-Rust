/**
 * @file
 * @brief Implementation of [merge
 * sort](https://en.wikipedia.org/wiki/Merge_sort) algorithm
 */
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
/**
 * @addtogroup sorting Sorting algorithms
 * @{
 */
/** Swap two integer variables
 * @param [in,out] a pointer to first variable
 * @param [in,out] b pointer to second variable
 */
void swap(int *a, int *b)
{
    int t;
    t = *a;
    *a = *b;
    *b = t;
}

/**
 * @brief Perform merge of segments.
 *
 * @param a array to sort
 * @param l left index for merge
 * @param r right index for merge
 * @param n total number of elements in the array
 */
void merge(int *a, int l, int r, int n)
{
    int *b = (int *)malloc(n * sizeof(int)); /* dynamic memory must be freed */
    if (b == NULL)
    {
        printf("Can't Malloc! Please try again.");
        exit(EXIT_FAILURE);
    }
    int c = l;
    int p1, p2;
    p1 = l;
    p2 = ((l + r) / 2) + 1;
    while ((p1 < ((l + r) / 2) + 1) && (p2 < r + 1))
    {
        if (a[p1] <= a[p2])
        {
            b[c++] = a[p1];
            p1++;
        }
        else
        {
            b[c++] = a[p2];
            p2++;
        }
    }

    if (p2 == r + 1)
    {
        while ((p1 < ((l + r) / 2) + 1))
        {
            b[c++] = a[p1];
            p1++;
        }
    }
    else
    {
        while ((p2 < r + 1))
        {
            b[c++] = a[p2];
            p2++;
        }
    }

    for (c = l; c < r + 1; c++) a[c] = b[c];

    free(b);
}

/** Merge sort algorithm implementation
 * @param a array to sort
 * @param n number of elements in the array
 * @param l index to sort from
 * @param r index to sort till
 */
void merge_sort(int *a, int n, int l, int r)
{
    if (r - l == 1)
    {
        if (a[l] > a[r])
            swap(&a[l], &a[r]);
    }
    else if (l != r)
    {
        merge_sort(a, n, l, (l + r) / 2);
        merge_sort(a, n, ((l + r) / 2) + 1, r);
        merge(a, l, r, n);
    }

    /* no change if l == r */
}
/** @} */

double test_with_100_values() {
    int a[100];
    srand(time(NULL));
    for (int i = 0; i < 100; i++) {
        a[i] = rand() % 100;
    }
    clock_t start = clock();
    merge_sort(a, 100, 0, 99);
    clock_t end = clock();
    double time_taken = ((double)(end - start)) / CLOCKS_PER_SEC;
    return time_taken;
}

/** Main function */
int main(void)
{
    // Run the test with 100 values
    double total_time = 0;
    for (int i = 0; i < 100; i++) {
        //printf("Test %d: ", i + 1);
        double time = test_with_100_values();
        total_time =total_time + time;
        //printf("%f\n", total_time);
    }

    double average_time = total_time / 100;
    printf("Average time taken to sort 100 arrays: %f seconds\n", average_time);

    return 0;
}