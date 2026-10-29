// Architecture data for DBX Enhanced visualization
export interface ArchitectureNode {
  id: string;
  type: string;
  position: { x: number; y: number };
  data: {
    label: string;
    description: string;
    category: 'core' | 'driver' | 'platform' | 'ai' | 'mcp' | 'plugin' | 'frontend' | 'external';
    crate?: string;
    features?: string[];
  };
}

export interface ArchitectureEdge {
  id: string;
  source: string;
  target: string;
  type?: string;
  label?: string;
  data?: {
    relationship: 'depends' | 'uses' | 'implements' | 'communicates' | 'extends';
  };
}

export const architectureNodes: ArchitectureNode[] = [
  // Core Layer
  {
    id: 'dbx-core',
    type: 'coreNode',
    position: { x: 400, y: 100 },
    data: {
      label: 'dbx-core',
      description: 'Application orchestration layer - connection routing, query orchestration, schema management, data import/export, AI agent loops, admin interfaces, persistence, safety, host integration',
      category: 'core',
      crate: 'dbx-core',
      features: ['connection', 'query', 'schema', 'data', 'ai', 'admin', 'persistence', 'safety', 'host'],
    },
  },

  // Type Layer
  {
    id: 'dbx-types',
    type: 'coreNode',
    position: { x: 200, y: 250 },
    data: {
      label: 'dbx-types',
      description: 'Connection config, database identity, query/metadata DTOs, JS-safe JSON, MQTT/GridFS data records',
      category: 'core',
      crate: 'dbx-types',
      features: ['config', 'identity', 'dto', 'json', 'mqtt', 'gridfs'],
    },
  },
  {
    id: 'dbx-platform',
    type: 'coreNode',
    position: { x: 400, y: 250 },
    data: {
      label: 'dbx-platform',
      description: 'Process management, paths, proxy, downloads, version comparison, shared user prompt gateway',
      category: 'platform',
      crate: 'dbx-platform',
      features: ['process', 'paths', 'proxy', 'downloads', 'version', 'prompts'],
    },
  },
  {
    id: 'dbx-sql',
    type: 'coreNode',
    position: { x: 600, y: 250 },
    data: {
      label: 'dbx-sql',
      description: 'SQL parsing, dialect registration/loading, SQL risk analysis, DDL/DML, structure diff planning',
      category: 'core',
      crate: 'dbx-sql',
      features: ['parse', 'dialect', 'risk', 'ddl', 'dml', 'diff'],
    },
  },
  {
    id: 'dbx-formats',
    type: 'coreNode',
    position: { x: 800, y: 250 },
    data: {
      label: 'dbx-formats',
      description: 'CSV/XLSX/text/time formatting, ZIP encoding',
      category: 'core',
      crate: 'dbx-formats',
      features: ['csv', 'xlsx', 'text', 'time', 'zip'],
    },
  },

  // Driver Layer
  {
    id: 'dbx-drivers',
    type: 'driverNode',
    position: { x: 400, y: 400 },
    data: {
      label: 'dbx-drivers',
      description: 'Native drivers, tunnels, JDBC/Agent runtime & distribution, timeout cancellation, SQLite metadata',
      category: 'driver',
      crate: 'dbx-drivers',
      features: ['native', 'tunnel', 'jdbc', 'agent', 'timeout', 'sqlite-meta'],
    },
  },
  {
    id: 'dbx-driver-postgres',
    type: 'driverNode',
    position: { x: 100, y: 500 },
    data: { label: 'dbx-driver-postgres', description: 'PostgreSQL native driver', category: 'driver', crate: 'dbx-driver-postgres' },
  },
  {
    id: 'dbx-driver-mysql',
    type: 'driverNode',
    position: { x: 200, y: 500 },
    data: { label: 'dbx-driver-mysql', description: 'MySQL/MariaDB native driver', category: 'driver', crate: 'dbx-driver-mysql' },
  },
  {
    id: 'dbx-driver-sqlite',
    type: 'driverNode',
    position: { x: 300, y: 500 },
    data: { label: 'dbx-driver-sqlite', description: 'SQLite native driver', category: 'driver', crate: 'dbx-sqlite-worker' },
  },
  {
    id: 'dbx-driver-redis',
    type: 'driverNode',
    position: { x: 400, y: 500 },
    data: { label: 'dbx-driver-redis', description: 'Redis native driver', category: 'driver', crate: 'dbx-driver-redis' },
  },
  {
    id: 'dbx-driver-mongodb',
    type: 'driverNode',
    position: { x: 500, y: 500 },
    data: { label: 'dbx-driver-mongodb', description: 'MongoDB native driver', category: 'driver', crate: 'dbx-driver-mongodb' },
  },
  {
    id: 'dbx-driver-sqlserver',
    type: 'driverNode',
    position: { x: 600, y: 500 },
    data: { label: 'dbx-driver-sqlserver', description: 'SQL Server native driver', category: 'driver', crate: 'dbx-driver-sqlserver' },
  },
  {
    id: 'dbx-driver-elasticsearch',
    type: 'driverNode',
    position: { x: 700, y: 500 },
    data: { label: 'dbx-driver-elasticsearch', description: 'Elasticsearch native driver', category: 'driver', crate: 'dbx-driver-elasticsearch' },
  },

  // African Domain Drivers (NEW)
  {
    id: 'dbx-african-domain',
    type: 'driverNode',
    position: { x: 850, y: 300 },
    data: {
      label: 'dbx-african-domain ★',
      description: 'Shared types for African fintech, banking, healthcare, logistics domains',
      category: 'driver',
      crate: 'dbx-african-domain',
      features: ['mobile-money', 'banking', 'healthcare', 'logistics', 'currency'],
    },
  },
  {
    id: 'dbx-driver-mobile-money',
    type: 'driverNode',
    position: { x: 850, y: 420 },
    data: {
      label: 'dbx-driver-mobile-money ★',
      description: 'M-Pesa Daraja, MTN MoMo, Airtel Money, Tigo Pesa, Orange Money, Vodacom M-Pesa, Wave, Chipper Cash, Paga, Opay, PalmPay',
      category: 'driver',
      crate: 'dbx-driver-mobile-money',
      features: ['mpesa', 'momo', 'airtel', 'tigo', 'orange', 'vodacom', 'wave', 'chipper', 'paga', 'opay', 'palmpay'],
    },
  },
  {
    id: 'dbx-driver-african-banking',
    type: 'driverNode',
    position: { x: 850, y: 500 },
    data: {
      label: 'dbx-driver-african-banking ★',
      description: 'NIBSS (Nigeria), Pesalink (Kenya), GHIPSS (Ghana), RTC (South Africa), Zimswitch (Zimbabwe), TISS (Tanzania), UIP (Uganda), IPPS (Rwanda)',
      category: 'driver',
      crate: 'dbx-driver-african-banking',
      features: ['nibss', 'pesalink', 'ghipss', 'rtc', 'zimswitch', 'tiss', 'uip', 'ipps'],
    },
  },
  {
    id: 'dbx-driver-dhis2',
    type: 'driverNode',
    position: { x: 850, y: 580 },
    data: {
      label: 'dbx-driver-dhis2 ★',
      description: 'DHIS2 analytics database connector for health data',
      category: 'driver',
      crate: 'dbx-driver-dhis2',
      features: ['analytics', 'data-elements', 'org-units', 'indicators'],
    },
  },
  {
    id: 'dbx-driver-openmrs',
    type: 'driverNode',
    position: { x: 850, y: 660 },
    data: {
      label: 'dbx-driver-openmrs ★',
      description: 'OpenMRS medical records connector for healthcare',
      category: 'driver',
      crate: 'dbx-driver-openmrs',
      features: ['patients', 'encounters', 'observations', 'concepts'],
    },
  },

  // SQL Layer
  {
    id: 'dbx-sql-core',
    type: 'coreNode',
    position: { x: 600, y: 350 },
    data: { label: 'dbx-sql-core', description: 'SQL core utilities', category: 'core', crate: 'dbx-sql-core' },
  },
  {
    id: 'dbx-sql-data',
    type: 'coreNode',
    position: { x: 700, y: 350 },
    data: { label: 'dbx-sql-data', description: 'SQL data operations', category: 'core', crate: 'dbx-sql-data' },
  },
  {
    id: 'dbx-sql-dialect',
    type: 'coreNode',
    position: { x: 500, y: 350 },
    data: { label: 'dbx-sql-dialect', description: 'SQL dialect handling', category: 'core', crate: 'dbx-sql-dialect' },
  },
  {
    id: 'dbx-sql-schema',
    type: 'coreNode',
    position: { x: 400, y: 350 },
    data: { label: 'dbx-sql-schema', description: 'SQL schema operations', category: 'core', crate: 'dbx-sql-schema' },
  },

  // AI & Plugin Layer
  {
    id: 'dbx-ai-provider',
    type: 'aiNode',
    position: { x: 100, y: 350 },
    data: {
      label: 'dbx-ai-provider',
      description: 'Model & CLI adapters, streaming protocol, events & token usage. Supports Claude, OpenAI, Ollama, local models',
      category: 'ai',
      crate: 'dbx-ai-provider',
      features: ['claude', 'openai', 'ollama', 'local', 'streaming', 'tokens'],
    },
  },
  {
    id: 'dbx-plugin-runtime',
    type: 'pluginNode',
    position: { x: 200, y: 350 },
    data: {
      label: 'dbx-plugin-runtime',
      description: 'Manifest, signatures & install, Marketplace, subprocess sessions & host requests',
      category: 'plugin',
      crate: 'dbx-plugin-runtime',
      features: ['manifest', 'signatures', 'marketplace', 'subprocess', 'host'],
    },
  },

  // MCP Server
  {
    id: 'dbx-mcp',
    type: 'mcpNode',
    position: { x: 100, y: 150 },
    data: {
      label: 'dbx-mcp ★',
      description: 'Model Context Protocol server (stdio + HTTP/SSE). 15+ African domain tools, benchmarking, architecture viz',
      category: 'mcp',
      crate: 'dbx-mcp',
      features: ['stdio', 'http', 'sse', 'african-tools', 'benchmark', 'architecture'],
    },
  },

  // Frontend Layer
  {
    id: 'src-tauri',
    type: 'frontendNode',
    position: { x: 400, y: 50 },
    data: {
      label: 'src-tauri (Desktop)',
      description: 'Tauri v2 + Rust + Vue/TypeScript. Cross-platform desktop app (macOS, Windows, Linux)',
      category: 'frontend',
      crate: 'src-tauri',
      features: ['tauri', 'vue', 'typescript', 'desktop', 'mobile'],
    },
  },
  {
    id: 'dbx-web',
    type: 'frontendNode',
    position: { x: 600, y: 50 },
    data: {
      label: 'dbx-web',
      description: 'Web version for browser-only environments. Same features as desktop.',
      category: 'frontend',
      crate: 'dbx-web',
      features: ['web', 'browser', 'docker', 'self-hosted'],
    },
  },
  {
    id: 'dbx-cli',
    type: 'frontendNode',
    position: { x: 200, y: 50 },
    data: {
      label: 'dbx-cli',
      description: 'CLI package for terminal, scripts, and Codex workflows. Includes Agent Skill.',
      category: 'frontend',
      crate: 'dbx-cli',
      features: ['cli', 'terminal', 'codex', 'agent-skill'],
    },
  },

  // Benchmark Suite (NEW)
  {
    id: 'dbx-bench',
    type: 'coreNode',
    position: { x: 600, y: 150 },
    data: {
      label: 'dbx-bench ★',
      description: 'Comprehensive benchmark suite with regression detection. Driver connect/query, MCP tools, AI SQL, data grid, schema diff, data transfer',
      category: 'core',
      crate: 'dbx-bench',
      features: ['driver', 'query', 'mcp', 'ai', 'grid', 'diff', 'transfer', 'regression'],
    },
  },

  // External Services
  {
    id: 'external-databases',
    type: 'externalNode',
    position: { x: 50, y: 550 },
    data: {
      label: '100+ Databases',
      description: 'PostgreSQL, MySQL, SQLite, Redis, MongoDB, ClickHouse, DuckDB, SQL Server, Oracle, Elasticsearch, and 90+ more',
      category: 'external',
      features: ['100+', 'native', 'jdbc', 'agent'],
    },
  },
  {
    id: 'external-ai',
    type: 'externalNode',
    position: { x: 50, y: 300 },
    data: {
      label: 'AI Providers',
      description: 'Claude, OpenAI, Ollama, Local LLMs, Any OpenAI-compatible endpoint',
      category: 'external',
      features: ['claude', 'openai', 'ollama', 'local', 'custom'],
    },
  },
  {
    id: 'external-mcp',
    type: 'externalNode',
    position: { x: 50, y: 150 },
    data: {
      label: 'MCP Clients',
      description: 'Claude Code, Cursor, Windsurf, Codex, Any MCP-compatible agent',
      category: 'external',
      features: ['claude-code', 'cursor', 'windsurf', 'codex'],
    },
  },
];

export const architectureEdges: ArchitectureEdge[] = [
  // Core dependencies
  { id: 'e1', source: 'dbx-core', target: 'dbx-types', data: { relationship: 'depends' } },
  { id: 'e2', source: 'dbx-core', target: 'dbx-platform', data: { relationship: 'depends' } },
  { id: 'e3', source: 'dbx-core', target: 'dbx-sql', data: { relationship: 'depends' } },
  { id: 'e4', source: 'dbx-core', target: 'dbx-formats', data: { relationship: 'depends' } },
  { id: 'e5', source: 'dbx-core', target: 'dbx-drivers', data: { relationship: 'depends' } },
  { id: 'e6', source: 'dbx-core', target: 'dbx-ai-provider', data: { relationship: 'depends' } },
  { id: 'e7', source: 'dbx-core', target: 'dbx-plugin-runtime', data: { relationship: 'depends' } },

  // SQL layer
  { id: 'e8', source: 'dbx-sql', target: 'dbx-sql-core', data: { relationship: 'depends' } },
  { id: 'e9', source: 'dbx-sql', target: 'dbx-sql-data', data: { relationship: 'depends' } },
  { id: 'e10', source: 'dbx-sql', target: 'dbx-sql-dialect', data: { relationship: 'depends' } },
  { id: 'e11', source: 'dbx-sql', target: 'dbx-sql-schema', data: { relationship: 'depends' } },

  // Driver layer
  { id: 'e12', source: 'dbx-drivers', target: 'dbx-driver-postgres', data: { relationship: 'implements' } },
  { id: 'e13', source: 'dbx-drivers', target: 'dbx-driver-mysql', data: { relationship: 'implements' } },
  { id: 'e14', source: 'dbx-drivers', target: 'dbx-driver-sqlite', data: { relationship: 'implements' } },
  { id: 'e15', source: 'dbx-drivers', target: 'dbx-driver-redis', data: { relationship: 'implements' } },
  { id: 'e16', source: 'dbx-drivers', target: 'dbx-driver-mongodb', data: { relationship: 'implements' } },
  { id: 'e17', source: 'dbx-drivers', target: 'dbx-driver-sqlserver', data: { relationship: 'implements' } },
  { id: 'e18', source: 'dbx-drivers', target: 'dbx-driver-elasticsearch', data: { relationship: 'implements' } },

  // African domain drivers
  { id: 'e19', source: 'dbx-drivers', target: 'dbx-african-domain', data: { relationship: 'depends' } },
  { id: 'e20', source: 'dbx-driver-mobile-money', target: 'dbx-african-domain', data: { relationship: 'depends' } },
  { id: 'e21', source: 'dbx-driver-african-banking', target: 'dbx-african-domain', data: { relationship: 'depends' } },
  { id: 'e22', source: 'dbx-driver-dhis2', target: 'dbx-african-domain', data: { relationship: 'depends' } },
  { id: 'e23', source: 'dbx-driver-openmrs', target: 'dbx-african-domain', data: { relationship: 'depends' } },

  // Frontend consumers
  { id: 'e24', source: 'src-tauri', target: 'dbx-core', data: { relationship: 'uses' } },
  { id: 'e25', source: 'dbx-web', target: 'dbx-core', data: { relationship: 'uses' } },
  { id: 'e26', source: 'dbx-cli', target: 'dbx-core', data: { relationship: 'uses' } },
  { id: 'e27', source: 'dbx-mcp', target: 'dbx-core', data: { relationship: 'uses' } },
  { id: 'e28', source: 'dbx-bench', target: 'dbx-core', data: { relationship: 'uses' } },
  { id: 'e29', source: 'dbx-bench', target: 'dbx-drivers', data: { relationship: 'uses' } },
  { id: 'e30', source: 'dbx-bench', target: 'dbx-mcp', data: { relationship: 'uses' } },
  { id: 'e31', source: 'dbx-bench', target: 'dbx-ai-provider', data: { relationship: 'uses' } },

  // External connections
  { id: 'e32', source: 'dbx-drivers', target: 'external-databases', data: { relationship: 'communicates' } },
  { id: 'e33', source: 'dbx-ai-provider', target: 'external-ai', data: { relationship: 'communicates' } },
  { id: 'e34', source: 'dbx-mcp', target: 'external-mcp', data: { relationship: 'communicates' } },
  { id: 'e35', source: 'dbx-driver-mobile-money', target: 'external-databases', data: { relationship: 'communicates' } },
  { id: 'e36', source: 'dbx-driver-african-banking', target: 'external-databases', data: { relationship: 'communicates' } },
  { id: 'e37', source: 'dbx-driver-dhis2', target: 'external-databases', data: { relationship: 'communicates' } },
  { id: 'e38', source: 'dbx-driver-openmrs', target: 'external-databases', data: { relationship: 'communicates' } },
];

export const categoryColors = {
  core: '#3b82f6',
  driver: '#10b981',
  platform: '#8b5cf6',
  ai: '#ec4899',
  mcp: '#f59e0b',
  plugin: '#6366f1',
  frontend: '#06b6d4',
  external: '#6b7280',
};

export const categoryLabels = {
  core: 'Core',
  driver: 'Drivers',
  platform: 'Platform',
  ai: 'AI',
  mcp: 'MCP',
  plugin: 'Plugin',
  frontend: 'Frontend',
  external: 'External',
};