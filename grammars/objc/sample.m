#import <Foundation/Foundation.h>

#define LIMIT 10

// A sample.
@protocol Shape <NSObject>
- (double)area;
@end

@interface Circle : NSObject <Shape>
@property (nonatomic, assign) double radius;
- (instancetype)initWithRadius:(double)radius;
@end

@implementation Circle

- (instancetype)initWithRadius:(double)radius {
    self = [super init];
    if (self) {
        _radius = radius;
    }
    return self;
}

- (double)area {
    return M_PI * self.radius * self.radius;
}

@end

int main(int argc, const char *argv[]) {
    @autoreleasepool {
        NSArray<Circle *> *shapes = @[[[Circle alloc] initWithRadius:1.0]];
        NSMutableDictionary *counts = [NSMutableDictionary dictionary];
        for (Circle *shape in shapes) {
            if ([shape area] < LIMIT) {
                counts[@"small"] = @(1);
            }
        }
        NSLog(@"%@ %d", counts, argc);
    }
    return 0;
}
