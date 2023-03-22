// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#import <Foundation/Foundation.h>

#import "TXSChildItem.h"
@interface TXSSubChildItem : TXSChildItem
- (nonnull instancetype)init NS_UNAVAILABLE;
+ (nonnull instancetype)new NS_UNAVAILABLE;
- (nonnull instancetype)initWithItems:(nonnull NSArray<NSString *> *)items
                               parent:(nonnull NSString *)parent
                                index:(int32_t)index NS_DESIGNATED_INITIALIZER;
+ (nonnull instancetype)subChildItemWithItems:(nonnull NSArray<NSString *> *)items
                                       parent:(nonnull NSString *)parent
                                        index:(int32_t)index;

@property (nonatomic, readonly) int32_t index;

@end
