/**
 * @file main.c
 * @author 临时试验代码
 * @brief
 * @version 0.1
 * @date 2022-04-19
 *
 * @copyright Copyright (c) 2022
 *
 */
#include <assert.h>
#include <stdio.h>

/**
 * @brief 计算数组 x[] 和 y[] 所有所引对应乘积的和
 * 代码来自：《C语言接口与实现》这本书
 * @param x
 * @param y
 * @param n
 * @return int
 */
int dotProduct(int x[], int y[], int n) {
  int i, sum;
  sum = 0;
  for (i = 0; i < n; i++)
    sum += x[i] * y[i];

  return sum;
}

int main(void) {
#define ARRAY_LEN 5
  int iarry1[ARRAY_LEN] = {1, 2, 3, 4, 5};
  int iarry2[ARRAY_LEN] = {6, 7, 8, 9, 10};

  printf("Sum: %d\n", dotProduct(iarry1, iarry2, ARRAY_LEN));

  assert(5 > 5);

  return 0;
}
