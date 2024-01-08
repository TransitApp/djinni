//
//  NSArray+Djnni.h
//  Djinni
//
//  Created by Julien Lepine on 2024-01-08.
//

@import Foundation;

NS_ASSUME_NONNULL_BEGIN

@interface NSArray <ObjectType>(Djinni)
;

@property (nonatomic, readonly) NSUInteger dynamicHash;

@end

NS_ASSUME_NONNULL_END
