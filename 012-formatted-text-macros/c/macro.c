#include <stdio.h>
#define PI 3.14
#define circleArea(r) (PI*r*r)

int main() {
	float radius, area;
	printf("Enter the radius: ");
	scanf("%f", &radius);

	area = circleArea(radius);

	printf("Area=%.2f\n", area);

	return 0;
}
