//
//  UIColor+Djinni.mm
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

#import "UIColor+Djinni.h"

@implementation UIColor (Djinni)

- (NSUInteger)dynamicHash {
    UIColor *lightColor = [self resolvedColorWithTraitCollection:[UITraitCollection traitCollectionWithUserInterfaceStyle:UIUserInterfaceStyleLight]];
    UIColor *darkColor = [self resolvedColorWithTraitCollection:[UITraitCollection traitCollectionWithUserInterfaceStyle:UIUserInterfaceStyleDark]];

    return [lightColor.rgbaHexStringFromColor stringByAppendingString:darkColor.rgbaHexStringFromColor].hash;
}

- (NSString *)rgbaHexStringFromColor {
    return [NSString stringWithFormat:@"%0.8X", self.rgbaHex];
}

- (UInt32)rgbaHex {
    CGFloat r, g, b, a;
    if (![self red:&r green:&g blue:&b alpha:&a])
        return 0;

    r = MIN(MAX(r, 0.0f), 1.0f);
    g = MIN(MAX(g, 0.0f), 1.0f);
    b = MIN(MAX(b, 0.0f), 1.0f);
    a = MIN(MAX(a, 0.0f), 1.0f);

    return (((uint)roundf(r * 255)) << 24)
           | (((uint)roundf(g * 255)) << 16)
           | (((uint)roundf(b * 255)) << 8)
           | (((uint)roundf(a * 255)));
}

- (BOOL)red:(CGFloat *)red green:(CGFloat *)green blue:(CGFloat *)blue alpha:(CGFloat *)alpha {
    const CGFloat *components = CGColorGetComponents(self.CGColor);

    CGFloat r, g, b, a;

    switch (CGColorSpaceGetModel(CGColorGetColorSpace(self.CGColor))) {
        case kCGColorSpaceModelMonochrome:
            r = g = b = components[0];
            a = components[1];
            break;
        case kCGColorSpaceModelRGB:
            r = components[0];
            g = components[1];
            b = components[2];
            a = components[3];
            break;
        default: // We don't know how to handle this model
            return NO;
    }

    if (red)
        *red = r;
    if (green)
        *green = g;
    if (blue)
        *blue = b;
    if (alpha)
        *alpha = a;

    return YES;
}

- (BOOL)isEqualToColor:(UIColor *)otherColor {
    UIColor *lightColor = [self resolvedColorWithTraitCollection:[UITraitCollection traitCollectionWithUserInterfaceStyle:UIUserInterfaceStyleLight]];
    UIColor *lightOtherColor = [otherColor resolvedColorWithTraitCollection:[UITraitCollection traitCollectionWithUserInterfaceStyle:UIUserInterfaceStyleLight]];
    UIColor *darkColor = [self resolvedColorWithTraitCollection:[UITraitCollection traitCollectionWithUserInterfaceStyle:UIUserInterfaceStyleDark]];
    UIColor *darkOtherColor = [otherColor resolvedColorWithTraitCollection:[UITraitCollection traitCollectionWithUserInterfaceStyle:UIUserInterfaceStyleDark]];

    return ([lightColor staticColorIsEqualToColor:lightOtherColor]
            && [darkColor staticColorIsEqualToColor:darkOtherColor]);
}

- (BOOL)staticColorIsEqualToColor:(UIColor *)otherColor {
    CGColorSpaceRef colorSpaceRGB = CGColorSpaceCreateDeviceRGB();

    UIColor * (^convertColorToRGBSpace)(UIColor *) = ^(UIColor *color) {
        if (CGColorSpaceGetModel(CGColorGetColorSpace(color.CGColor)) == kCGColorSpaceModelMonochrome) {
            const CGFloat *oldComponents = CGColorGetComponents(color.CGColor);
            CGFloat components[4] = {oldComponents[0], oldComponents[0], oldComponents[0], oldComponents[1]};

            CGColorRef colorRef = CGColorCreate(colorSpaceRGB, components);
            UIColor *resultColor = [UIColor colorWithCGColor:colorRef];
            CGColorRelease(colorRef);

            return resultColor;
        }
        else
            return color;
    };

    UIColor *selfColor = convertColorToRGBSpace(self);
    otherColor = convertColorToRGBSpace(otherColor);
    CGColorSpaceRelease(colorSpaceRGB);

    return [selfColor isEqual:otherColor];
}

@end
