import { Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { TypeOrmModule } from '@nestjs/typeorm';
import { CacheModule } from '@nestjs/cache-manager';
import { ThrottlerModule } from '@nestjs/throttler';
import { BullModule } from '@nestjs/bull';
import * as redisStore from 'cache-manager-redis-store';

// Core modules
import { AuthModule } from './auth/auth.module';
import { GameModule } from './game/game.module';
import { EconomyModule } from './economy/economy.module';
import { DaoModule } from './dao/dao.module';
import { NotificationsModule } from './notifications/notifications.module';

// Common modules
import { DatabaseModule } from './database/database.module';
import { CommonModule } from './common/common.module';

// Configuration
import { databaseConfig } from './config/database.config';
import { redisConfig } from './config/redis.config';
import { solanaConfig } from './config/solana.config';

@Module({
  imports: [
    // Configuration
    ConfigModule.forRoot({
      isGlobal: true,
      load: [databaseConfig, redisConfig, solanaConfig],
      envFilePath: ['.env.local', '.env'],
    }),

    // Database
    TypeOrmModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: (configService: ConfigService) => ({
        type: 'postgres',
        host: configService.get('database.host'),
        port: configService.get('database.port'),
        username: configService.get('database.username'),
        password: configService.get('database.password'),
        database: configService.get('database.name'),
        entities: [__dirname + '/**/*.entity{.ts,.js}'],
        synchronize: configService.get('NODE_ENV') !== 'production',
        logging: configService.get('NODE_ENV') === 'development',
        ssl: configService.get('NODE_ENV') === 'production' ? { rejectUnauthorized: false } : false,
      }),
      inject: [ConfigService],
    }),

    // Redis Cache
    CacheModule.registerAsync({
      imports: [ConfigModule],
      useFactory: async (configService: ConfigService) => ({
        store: redisStore,
        host: configService.get('redis.host'),
        port: configService.get('redis.port'),
        password: configService.get('redis.password'),
        ttl: 300, // 5 minutes default TTL
      }),
      inject: [ConfigService],
      isGlobal: true,
    }),

    // Rate Limiting
    ThrottlerModule.forRoot({
      ttl: 60, // 1 minute
      limit: 100, // 100 requests per minute
    }),

    // Bull Queue
    BullModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: async (configService: ConfigService) => ({
        redis: {
          host: configService.get('redis.host'),
          port: configService.get('redis.port'),
          password: configService.get('redis.password'),
        },
      }),
      inject: [ConfigService],
    }),

    // Application modules
    DatabaseModule,
    CommonModule,
    AuthModule,
    GameModule,
    EconomyModule,
    DaoModule,
    NotificationsModule,
  ],
  controllers: [],
  providers: [],
})
export class AppModule {}
