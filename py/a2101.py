from prelude import *

if __name__ == '__main__':
    nums = read(int)

    n = 0
    for i in range(1, len(nums)):
        if nums[i] > nums[i - 1]:
            n += 1
    print(n)

    nums2 = [nums[i] + nums[i-1] + nums[i-2] for i in range(2, len(nums))]
    n = 0
    for i in range(1, len(nums2)):
        if nums2[i] > nums2[i - 1]:
            n += 1
    print(n)
