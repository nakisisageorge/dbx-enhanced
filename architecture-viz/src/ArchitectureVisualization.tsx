import React, { useState, useCallback } from 'react';
import ReactFlow, {
  Node,
  Edge,
  addEdge,
  Connection,
  NodeTypes,
  EdgeTypes,
  Background,
  Controls,
  MiniMap,
  Panel,
  useReactFlow,
  fitView,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { architectureNodes, architectureEdges, categoryColors, categoryLabels } from '../data/architecture';
import { ArchitectureNode, ArchitectureEdge } from '../data/architecture';
import CoreNode from './CoreNode';
import DriverNode from './DriverNode';
import PlatformNode from './PlatformNode';
import AiNode from './AiNode';
import McpNode from './McpNode';
import PluginNode from './PluginNode';
import FrontendNode from './FrontendNode';
import ExternalNode from './ExternalNode';

const initialNodes: Node[] = architectureNodes.map((n) => ({
  id: n.id,
  type: n.type,
  position: n.position,
  data: n.data,
}));

const initialEdges: Edge[] = architectureEdges.map((e) => ({
  id: e.id,
  source: e.source,
  target: e.target,
  type: e.type || 'smoothstep',
  label: e.label,
  data: e.data,
  animated: true,
  style: { stroke: '#94a3b8', strokeWidth: 1.5 },
  markerEnd: { type: 'arrowclosed', color: '#94a3b8' },
}));

const nodeTypes: NodeTypes = {
  coreNode: CoreNode,
  driverNode: DriverNode,
  platformNode: PlatformNode,
  aiNode: AiNode,
  mcpNode: McpNode,
  pluginNode: PluginNode,
  frontendNode: FrontendNode,
  externalNode: ExternalNode,
};

const edgeTypes: EdgeTypes = {
  smoothstep: {
    labelBgStyle: { fill: '#1e293b', fillOpacity: 0.8 },
    labelBgPadding: [2, 4],
    labelBgBorderRadius: 2,
  },
};

function ArchitectureVisualization() {
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [filterCategory, setFilterCategory] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState('');
  const { getNodes, setNodes, getEdges, setEdges, viewport, setViewport } = useReactFlow();

  const filteredNodes = useCallback(() => {
    const nodes = getNodes();
    return nodes.filter((node) => {
      if (filterCategory !== 'all' && node.data.category !== filterCategory) return false;
      if (searchQuery && !node.data.label.toLowerCase().includes(searchQuery.toLowerCase())) return false;
      return true;
    });
  }, [getNodes, filterCategory, searchQuery]);

  const handleNodeClick = useCallback((event: React.MouseEvent, node: Node) => {
    setSelectedNode(node);
  }, []);

  const handleFitView = () => {
    fitView({ nodes: getNodes(), edges: getEdges() }, 50);
  };

  const categories = ['all', 'core', 'driver', 'platform', 'ai', 'mcp', 'plugin', 'frontend', 'external'];

  return (
    <div style={{ width: '100%', height: '100vh', background: '#0f172a' }}>
      {/* Header */}
      <div style={{
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
        padding: '16px 24px',
        background: 'linear-gradient(135deg, #0f172a 0%, #1e293b 100%)',
        borderBottom: '1px solid #334155',
        zIndex: 100,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        flexWrap: 'wrap',
        gap: '16px',
      }}>
        <div>
          <h1 style={{ margin: 0, fontSize: '24px', fontWeight: 700, color: '#f8fafc' }}>
            DBX Enhanced Architecture
          </h1>
          <p style={{ margin: '4px 0 0', fontSize: '14px', color: '#94a3e8' }}>
            Interactive architecture explorer — Rust/Tauri cross-platform database client with African domain extensions
          </p>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
          <div style={{ position: 'relative' }}>
            <input
              type="text"
              placeholder="Search nodes..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              style={{
                padding: '8px 16px 8px 40px',
                borderRadius: '8px',
                border: '1px solid #334155',
                background: '#1e293b',
                color: '#f8fafc',
                width: '250px',
                fontSize: '14px',
              }}
            />
            <svg style={{ position: 'absolute', left: '12px', top: '50%', transform: 'translateY(-50%)', width: '16px', height: '16px', color: '#64748b' }} fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
          <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
            {categories.map((cat) => (
              <button
                key={cat}
                onClick={() => setFilterCategory(cat)}
                style={{
                  padding: '6px 12px',
                  borderRadius: '6px',
                  border: 'none',
                  background: filterCategory === cat ? categoryColors[cat as keyof typeof categoryColors] : '#1e293b',
                  color: '#f8fafc',
                  fontSize: '12px',
                  fontWeight: 500,
                  cursor: 'pointer',
                  transition: 'all 0.2s',
                }}
              >
                {cat === 'all' ? 'All' : categoryLabels[cat as keyof typeof categoryLabels]}
              </button>
            ))}
          </div>
          <button
            onClick={handleFitView}
            style={{
              padding: '8px 16px',
              borderRadius: '8px',
              border: '1px solid #334155',
              background: '#1e293b',
              color: '#f8fafc',
              fontSize: '14px',
              cursor: 'pointer',
            }}
          >
            Fit View
          </button>
        </div>
      </div>

      {/* React Flow */}
      <ReactFlow
        nodes={initialNodes}
        edges={initialEdges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        onNodeClick={handleNodeClick}
        onConnect={(connection: Connection) => setEdges((eds) => addEdge(connection, eds))}
        fitView={false}
        attributionPosition="bottom-right"
      >
        <Background color="#334155" gap={16} />
        <Controls />
        <MiniMap
          nodeColor={(node) => categoryColors[node.data.category as keyof typeof categoryColors] || '#6b7280'}
          maskColor="rgba(15, 23, 42, 0.8)"
        />
        <Panel position="bottom-left">
          <div style={{
            background: '#1e293b',
            border: '1px solid #334155',
            borderRadius: '8px',
            padding: '12px',
            minWidth: '200px',
            color: '#f8fafc',
            fontSize: '12px',
          }}>
            <strong>Legend</strong>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px', marginTop: '8px' }}>
              {Object.entries(categoryLabels).map(([key, label]) => (
                <div key={key} style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                  <div style={{
                    width: '12px',
                    height: '12px',
                    borderRadius: '3px',
                    background: categoryColors[key as keyof typeof categoryColors],
                  }} />
                  <span>{label}</span>
                </div>
              ))}
            </div>
          </div>
        </Panel>
      </ReactFlow>

      {/* Node Details Panel */}
      {selectedNode && (
        <div style={{
          position: 'absolute',
          top: '100px',
          right: '24px',
          bottom: '24px',
          width: '380px',
          background: '#1e293b',
          border: '1px solid #334155',
          borderRadius: '12px',
          padding: '16px',
          overflow: 'auto',
          zIndex: 50,
          boxShadow: '0 20px 40px rgba(0,0,0,0.4)',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '12px' }}>
            <h3 style={{ margin: 0, fontSize: '16px', fontWeight: 600, color: '#f8fafc' }}>
              {selectedNode.data.label}
            </h3>
            <button
              onClick={() => setSelectedNode(null)}
              style={{
                background: 'none',
                border: 'none',
                color: '#94a3b8',
                cursor: 'pointer',
                padding: '4px',
              }}
            >
              ✕
            </button>
          </div>
          <div style={{ marginBottom: '12px', paddingBottom: '12px', borderBottom: '1px solid #334155' }}>
            <span style={{
              display: 'inline-block',
              padding: '2px 8px',
              borderRadius: '4px',
              background: categoryColors[selectedNode.data.category as keyof typeof categoryColors] + '20',
              color: categoryColors[selectedNode.data.category as keyof typeof categoryColors],
              fontSize: '11px',
              fontWeight: 500,
              textTransform: 'uppercase',
            }}>
              {categoryLabels[selectedNode.data.category as keyof typeof categoryLabels]}
            </span>
          </div>
          <p style={{ color: '#cbd5e1', fontSize: '13px', lineHeight: '1.5', marginBottom: '16px' }}>
            {selectedNode.data.description}
          </p>
          {selectedNode.data.crate && (
            <div style={{ marginBottom: '12px' }}>
              <strong style={{ color: '#94a3b8', fontSize: '11px', textTransform: 'uppercase' }}>Crate</strong>
              <code style={{ display: 'block', marginTop: '4px', padding: '4px 8px', background: '#0f172a', borderRadius: '4px', color: '#a5b4fc', fontSize: '12px' }}>
                {selectedNode.data.crate}
              </code>
            </div>
          )}
          {selectedNode.data.features && selectedNode.data.features.length > 0 && (
            <div>
              <strong style={{ color: '#94a3b8', fontSize: '11px', textTransform: 'uppercase' }}>Features</strong>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '8px' }}>
                {selectedNode.data.features.map((feature) => (
                  <span key={feature} style={{
                    padding: '4px 10px',
                    borderRadius: '999px',
                    background: '#0f172a',
                    border: '1px solid #334155',
                    color: '#cbd5e1',
                    fontSize: '11px',
                  }}>
                    {feature}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default ArchitectureVisualization;